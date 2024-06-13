use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Pll",
            extends: None,
            description: Some(
                "no description available.",
            ),
            items: &[
                BlockItem {
                    name: "cfg0",
                    description: Some(
                        "PLLx config0.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg1",
                    description: Some(
                        "PLLx config1.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfg2",
                    description: Some(
                        "PLLx config2.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfg2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "freq",
                    description: Some(
                        "PLLx frac mode frequency adjust.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Freq",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lock",
                    description: Some(
                        "PLLx lock control.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lock",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "status",
                    description: Some(
                        "PLLx status.",
                    ),
                    array: None,
                    byte_offset: 0x20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Status",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "div0",
                    description: Some(
                        "PLLx divider0 control.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Div0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "div1",
                    description: Some(
                        "PLLx divider1 control.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Div1",
                            ),
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Pllctl",
            extends: None,
            description: Some(
                "PLLCTL.",
            ),
            items: &[
                BlockItem {
                    name: "xtal",
                    description: Some(
                        "Crystal control and status.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Xtal",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pll",
                    description: Some(
                        "no description available.",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 5,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 0x80,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Pll",
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfg0",
            extends: None,
            description: Some(
                "PLLx config0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsmpd",
                    description: Some(
                        "1: int mode; 0: frac mode.",
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
                    name: "ss_disable_sscg",
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
                    name: "ss_reset",
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
                    name: "ss_downspread",
                    description: Some(
                        "Downspread control 1’b0 –> Center-Spread 1’b1 –> Downspread.",
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
                    name: "ss_divval",
                    description: Some(
                        "sscg divval, lock when lock_en[8]&~pll_ana_pd.",
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
                    name: "ss_spread",
                    description: Some(
                        "lock when lock_en[14]&~pll_ana_pd.",
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
                    name: "postdiv1",
                    description: Some(
                        "lock when lock_en[20]&~pll_ana_pd.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "refdiv",
                    description: Some(
                        "refclk diverder, lock when lock_en[24]&~pll_ana_pd.",
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
                Field {
                    name: "ss_rstptr",
                    description: Some(
                        "reset pointer, for sscg, lock when lock_en[31]&~pll_ana_pd&~pll_lock_comb.",
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
            name: "Cfg1",
            extends: None,
            description: Some(
                "PLLx config1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock_cnt_cfg",
                    description: Some(
                        "used to wait lock if set larger than lock time; default 1500 24M cycle if refdiv is 1, 4500 cycle if refdiv is 3.",
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
                    name: "pllpd_sw",
                    description: Some(
                        "pll power down. pll_ana_pd = pllctrl_hw_en ? (pll_pd_soc|pll_pd_chg) : pllpd_sw; pll_pd_soc is just delay of soc enable, for soc to control pll on/off; pll_pd_chg is used to power down pll when div_chg_mode is 1, if software update pll parameter(fbdiv or frac), pll_ctrl will power down pll, update parameter, then power up pll. response to soc will not de-asserted at this sequence.",
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
                    name: "clken_sw",
                    description: Some(
                        "the clock enable used to gate pll output, should be set after lock, and clear before power down pll. pll_clock_enable = pllctrl_hw_en ? (pll_lock_comb & enable & pll_clk_enable_chg) : clken_sw;.",
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
                    name: "pllctrl_hw_en",
                    description: Some(
                        "1: hardware controll PLL settings, software can update register, but result unknown; suggested only update fbdiv and frac value 0: full software control PLL settings.",
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
            name: "Cfg2",
            extends: None,
            description: Some(
                "PLLx config2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fbdiv_int",
                    description: Some(
                        "fbdiv used in int mode.",
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
            name: "Div0",
            extends: None,
            description: Some(
                "PLLx divider0 control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256.",
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
                        "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on.",
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
                    name: "response",
                    description: Some(
                        "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use.",
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
                    name: "busy",
                    description: Some(
                        "Busy flag 0: divider is working 1: divider is changing status.",
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
            name: "Div1",
            extends: None,
            description: Some(
                "PLLx divider1 control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "div",
                    description: Some(
                        "Divider 0: divide by 1 1: divide by2 . . . 255: divide by 256.",
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
                        "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on.",
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
                    name: "response",
                    description: Some(
                        "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use.",
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
                    name: "busy",
                    description: Some(
                        "Busy flag 0: divider is working 1: divider is changing status.",
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
            name: "Freq",
            extends: None,
            description: Some(
                "PLLx frac mode frequency adjust.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fbdiv_frac",
                    description: Some(
                        "fbdiv used in frac mode.",
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
                    name: "frac",
                    description: Some(
                        "PLL output frequency is : Fout = Fref/refdiv*(fbdiv + frac/2^24)/postdiv1 for default refdiv=1 and postdiv1=1, 24MHz refclk Fout is 24*fbdiv in int mode if frac is set to 0x800000, Fout is 24*(fbdiv+0.5) Fout is 24*fbdiv in int mode if frac is set to 0x200000, Fout is 24*(fbdiv+0.125).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 24,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lock",
            extends: None,
            description: Some(
                "PLLx lock control.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lock_ss_divval",
                    description: Some(
                        "lock bit of field ss_divval 0: field is open foe software to change 1: field is locked, not changeable.",
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
                    name: "lock_ss_spead",
                    description: Some(
                        "lock bit of field ss_spead 0: field is open foe software to change 1: field is locked, not changeable.",
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
                    name: "lock_postdiv1",
                    description: Some(
                        "lock bit of field postdiv1 0: field is open foe software to change 1: field is locked, not changeable.",
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
                    name: "lock_refdiv",
                    description: Some(
                        "lock bit of field refdiv 0: field is open foe software to change 1: field is locked, not changeable.",
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
                    name: "lock_ss_rstptr",
                    description: Some(
                        "lock bit of field ss_rstptr 0: field is open foe software to change 1: field is locked, not changeable.",
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
            name: "Status",
            extends: None,
            description: Some(
                "PLLx status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pll_lock_sync",
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
                    name: "pll_lock_comb",
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
                    name: "response",
                    description: Some(
                        "response to SYSCTL, PLL is power down when both enable and response are 0.",
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
                    name: "enable",
                    description: Some(
                        "enable from SYSCTL block.",
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
            name: "Xtal",
            extends: None,
            description: Some(
                "Crystal control and status.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ramp_time",
                    description: Some(
                        "Rampup time of XTAL oscillator in cycles of IRC24M clock 0: 0 cycle 1: 1 cycle 2: 2 cycle 1048575: 1048575 cycles.",
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
                    name: "enable",
                    description: Some(
                        "Crystal oscillator enable status 0: Oscillator is off 1: Oscillator is on.",
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
                    name: "response",
                    description: Some(
                        "Crystal oscillator status 0: Oscillator is not stable 1: Oscillator is stable for use.",
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
    ],
    enums: &[],
};
