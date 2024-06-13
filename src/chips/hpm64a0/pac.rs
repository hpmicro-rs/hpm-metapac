#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {}
unsafe impl crate::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors {
    extern "C" {}
    pub union Vector {
        _handler: unsafe extern "C" fn(),
        _reserved: u32,
    }
    #[link_section = ".vector_table.interrupts"]
    #[no_mangle]
    pub static __VECTORED_INTERRUPTS: [Vector; 0] = [];
}
pub const PLIC: plic::Plic = unsafe { plic::Plic::from_ptr(0xe400_0000usize as _) };
pub const MCHTMR: mchtmr::Mchtmr = unsafe { mchtmr::Mchtmr::from_ptr(0xe600_0000usize as _) };
pub const PLICSW: plicsw::Plicsw = unsafe { plicsw::Plicsw::from_ptr(0xe640_0000usize as _) };
#[cfg(feature = "rt")]
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[path = "../../peripherals/mchtmr_common.rs"]
pub mod mchtmr;
#[path = "../../peripherals/plic_common.rs"]
pub mod plic;
#[path = "../../peripherals/plicsw_common.rs"]
pub mod plicsw;
pub const CORE_INDEX: usize = 0;
pub const FLASH_BASE: usize = 2147483648;
pub const FLASH_SIZE: usize = 1048576;
pub mod resources {
    pub const KEYM: usize = 272;
    pub const MBX0: usize = 276;
    pub const CLK_TOP_UART12: usize = 95;
    pub const UART13: usize = 303;
    pub const CLK_TOP_GPTMR1: usize = 76;
    pub const CLK_TOP_UART1: usize = 84;
    pub const CLK_TOP_I2S3: usize = 199;
    pub const UART11: usize = 301;
    pub const SPI1: usize = 311;
    pub const CLK_TOP_SPI2: usize = 105;
    pub const UART0: usize = 290;
    pub const ADC0: usize = 319;
    pub const CLK_TOP_UART3: usize = 86;
    pub const UART2: usize = 292;
    pub const UART8: usize = 298;
    pub const CLK_TOP_I2S1: usize = 197;
    pub const PDM: usize = 328;
    pub const USB1: usize = 347;
    pub const CLK_TOP_CAN2: usize = 109;
    pub const CAN0: usize = 314;
    pub const I2S1: usize = 325;
    pub const POW_CON: usize = 21;
    pub const CLK_SRC_PLL2: usize = 38;
    pub const ADC2: usize = 321;
    pub const UART14: usize = 304;
    pub const WDG2: usize = 280;
    pub const RST_CPU0: usize = 28;
    pub const CLK_TOP_PTP1: usize = 124;
    pub const CLK_SRC_PLL1: usize = 35;
    pub const GPTMR1: usize = 283;
    pub const CLK_TOP_UART14: usize = 97;
    pub const CLK_TOP_REF0: usize = 125;
    pub const I2C0: usize = 306;
    pub const LCDC: usize = 335;
    pub const CLK_TOP_ANA2: usize = 114;
    pub const CLK_TOP_ADC3: usize = 195;
    pub const CLK_SRC_PLL4CLK0: usize = 44;
    pub const SDXC0: usize = 344;
    pub const REF0: usize = 348;
    pub const AHBAPB_BUS: usize = 256;
    pub const CLK_TOP_I2C3: usize = 102;
    pub const CLK_TOP_SDXC1: usize = 130;
    pub const I2C3: usize = 309;
    pub const UART6: usize = 296;
    pub const CLK_TOP_UART9: usize = 92;
    pub const CLK_SRC_PLL2CLK1: usize = 40;
    pub const I2C2: usize = 308;
    pub const MCHTMR1: usize = 265;
    pub const CAN3: usize = 317;
    pub const CLK_TOP_UART6: usize = 89;
    pub const CAM0: usize = 336;
    pub const MOT1: usize = 332;
    pub const CLK_SRC_PLL1CLK0: usize = 36;
    pub const CLK_TOP_CONN: usize = 69;
    pub const CLK_TOP_ENET0: usize = 121;
    pub const CLK_TOP_AUD1: usize = 116;
    pub const SPI3: usize = 313;
    pub const CLK_TOP_CAM0: usize = 119;
    pub const CLK_SRC_PLL1CLK1: usize = 37;
    pub const CLK_TOP_SPI3: usize = 106;
    pub const CLK_TOP_NTMR1: usize = 128;
    pub const CLK_SRC_XTAL: usize = 32;
    pub const CLK_TOP_SPI1: usize = 104;
    pub const CLK_TOP_ADC1: usize = 193;
    pub const GPTMR3: usize = 285;
    pub const GPTMR5: usize = 287;
    pub const CLK_TOP_I2S0: usize = 196;
    pub const CPU0_CORE: usize = 0;
    pub const WDG3: usize = 281;
    pub const GPTMR4: usize = 286;
    pub const I2S3: usize = 327;
    pub const CLK_SRC_PLL2CLK0: usize = 39;
    pub const CAN2: usize = 316;
    pub const DAO: usize = 329;
    pub const CLK_TOP_PTP0: usize = 123;
    pub const MOT0: usize = 331;
    pub const CLK_SRC_PLL0: usize = 33;
    pub const SPI2: usize = 312;
    pub const CLK_TOP_AHB: usize = 71;
    pub const CLK_TOP_CAN0: usize = 107;
    pub const RST_SOC: usize = 25;
    pub const CLK_TOP_UART15: usize = 98;
    pub const MOT2: usize = 333;
    pub const CLK_TOP_GPTMR0: usize = 75;
    pub const UART4: usize = 294;
    pub const ACMP: usize = 323;
    pub const CLK_TOP_ADC0: usize = 192;
    pub const CLK_TOP_FEMC: usize = 72;
    pub const CPU0_SUBSYS: usize = 1;
    pub const WDG0: usize = 278;
    pub const MBX1: usize = 277;
    pub const CLK_TOP_UART11: usize = 94;
    pub const CLK_TOP_MCHTMR0: usize = 65;
    pub const CLK_TOP_LCDC: usize = 118;
    pub const UART1: usize = 291;
    pub const NTMR1: usize = 343;
    pub const CAN1: usize = 315;
    pub const XPI0: usize = 268;
    pub const CLK_TOP_ADC2: usize = 194;
    pub const POW_CPU0: usize = 23;
    pub const JPEG: usize = 338;
    pub const CLK_TOP_GPTMR2: usize = 77;
    pub const CLK_TOP_PTPC: usize = 111;
    pub const SPI0: usize = 310;
    pub const GPTMR0: usize = 282;
    pub const CLK_TOP_I2C1: usize = 100;
    pub const CLK_TOP_I2C2: usize = 101;
    pub const RST_CPU1: usize = 29;
    pub const WDG1: usize = 279;
    pub const ENET1: usize = 341;
    pub const CLK_TOP_CPU0: usize = 64;
    pub const PTPC: usize = 318;
    pub const UART3: usize = 293;
    pub const CLK_TOP_I2C0: usize = 99;
    pub const SYNT: usize = 330;
    pub const CLK_TOP_UART13: usize = 96;
    pub const CLK_SRC_PLL0CLK0: usize = 34;
    pub const CLK_TOP_REF1: usize = 126;
    pub const AXI_SRAM1: usize = 267;
    pub const FEMC: usize = 260;
    pub const GPTMR6: usize = 288;
    pub const UART10: usize = 300;
    pub const CLK_SRC_PLL4: usize = 43;
    pub const CLK_TOP_CPU1: usize = 66;
    pub const ROM: usize = 261;
    pub const UART12: usize = 302;
    pub const CPU1_CORE: usize = 8;
    pub const MOT3: usize = 334;
    pub const GPTMR2: usize = 284;
    pub const XDMA: usize = 274;
    pub const AXI_SRAM0: usize = 266;
    pub const CLK_TOP_UART0: usize = 83;
    pub const RST_VIS: usize = 27;
    pub const CLK_TOP_AXI: usize = 68;
    pub const CLK_SRC_PLL3: usize = 41;
    pub const AXI_BUS: usize = 257;
    pub const LMM1: usize = 263;
    pub const CLK_TOP_UART5: usize = 88;
    pub const CLK_TOP_AUD0: usize = 115;
    pub const POW_CPU1: usize = 24;
    pub const CLK_TOP_GPTMR7: usize = 82;
    pub const USB0: usize = 346;
    pub const ADC1: usize = 320;
    pub const SDP: usize = 270;
    pub const I2S2: usize = 326;
    pub const HDMA: usize = 273;
    pub const CLK_TOP_GPTMR6: usize = 81;
    pub const CLK_TOP_UART4: usize = 87;
    pub const CLK_TOP_SDXC0: usize = 129;
    pub const CLK_TOP_UART8: usize = 91;
    pub const CLK_TOP_I2S2: usize = 198;
    pub const VIS_BUS: usize = 259;
    pub const PDMA: usize = 339;
    pub const CLK_TOP_AUD2: usize = 117;
    pub const NTMR0: usize = 342;
    pub const CPX1_SUBSYS: usize = 9;
    pub const GPTMR7: usize = 289;
    pub const GPIO: usize = 275;
    pub const CLK_SRC_PLL3CLK0: usize = 42;
    pub const CLK_TOP_UART10: usize = 93;
    pub const CLK_TOP_XPI0: usize = 73;
    pub const CLK_TOP_CAN3: usize = 110;
    pub const RNG: usize = 271;
    pub const CLK_TOP_ANA1: usize = 113;
    pub const CLK_TOP_XPI1: usize = 74;
    pub const UART9: usize = 299;
    pub const SDXC1: usize = 345;
    pub const I2S0: usize = 324;
    pub const CLK_TOP_GPTMR3: usize = 78;
    pub const CLK_TOP_UART2: usize = 85;
    pub const CLK_TOP_MCHTMR1: usize = 67;
    pub const I2C1: usize = 307;
    pub const CLK_TOP_GPTMR5: usize = 80;
    pub const CLK_TOP_UART7: usize = 90;
    pub const RST_CON: usize = 26;
    pub const LMM0: usize = 262;
    pub const UART15: usize = 305;
    pub const CONN_BUS: usize = 258;
    pub const CAM1: usize = 337;
    pub const REF1: usize = 349;
    pub const XPI1: usize = 269;
    pub const UART7: usize = 297;
    pub const POW_VIS: usize = 22;
    pub const UART5: usize = 295;
    pub const CLK_TOP_VIS: usize = 70;
    pub const CLK_TOP_ANA0: usize = 112;
    pub const CLK_TOP_NTMR0: usize = 127;
    pub const MCHTMR0: usize = 264;
    pub const ADC3: usize = 322;
    pub const ENET0: usize = 340;
    pub const CLK_TOP_ENET1: usize = 122;
    pub const CLK_TOP_SPI0: usize = 103;
    pub const CLK_TOP_GPTMR4: usize = 79;
    pub const CLK_TOP_CAN1: usize = 108;
    pub const CLK_TOP_CAM1: usize = 120;
}
pub mod clocks {
    pub const SPI0: usize = 39;
    pub const CAN3: usize = 46;
    pub const I2C0: usize = 35;
    pub const XPI0: usize = 9;
    pub const ENET1: usize = 58;
    pub const PTP1: usize = 60;
    pub const SPI1: usize = 40;
    pub const GPTMR2: usize = 13;
    pub const UART14: usize = 33;
    pub const FEMC: usize = 8;
    pub const XPI1: usize = 10;
    pub const GPTMR6: usize = 17;
    pub const GPTMR0: usize = 11;
    pub const AUD0: usize = 51;
    pub const REF0: usize = 61;
    pub const GPTMR7: usize = 18;
    pub const MCHTMR0: usize = 1;
    pub const I2C3: usize = 38;
    pub const CPU1: usize = 2;
    pub const UART9: usize = 28;
    pub const UART7: usize = 26;
    pub const ANA0: usize = 48;
    pub const UART15: usize = 34;
    pub const CAN2: usize = 45;
    pub const CAN1: usize = 44;
    pub const AUD2: usize = 53;
    pub const AXI: usize = 4;
    pub const GPTMR1: usize = 12;
    pub const UART13: usize = 32;
    pub const UART4: usize = 23;
    pub const NTMR1: usize = 64;
    pub const SPI3: usize = 42;
    pub const PTPC: usize = 47;
    pub const I2C2: usize = 37;
    pub const ENET0: usize = 57;
    pub const GPTMR3: usize = 14;
    pub const NTMR0: usize = 63;
    pub const UART8: usize = 27;
    pub const SPI2: usize = 41;
    pub const AHB: usize = 7;
    pub const ANA1: usize = 49;
    pub const CAM0: usize = 55;
    pub const GPTMR4: usize = 15;
    pub const UART3: usize = 22;
    pub const UART5: usize = 24;
    pub const SDXC1: usize = 66;
    pub const PTP0: usize = 59;
    pub const CPU0: usize = 0;
    pub const UART12: usize = 31;
    pub const UART11: usize = 30;
    pub const GPTMR5: usize = 16;
    pub const REF1: usize = 62;
    pub const UART0: usize = 19;
    pub const LCDC: usize = 54;
    pub const CAM1: usize = 56;
    pub const VIS: usize = 6;
    pub const UART2: usize = 21;
    pub const MCHTMR: usize = 3;
    pub const I2C1: usize = 36;
    pub const ANA2: usize = 50;
    pub const AUD1: usize = 52;
    pub const UART1: usize = 20;
    pub const SDXC0: usize = 65;
    pub const UART6: usize = 25;
    pub const CAN0: usize = 43;
    pub const CONN: usize = 5;
    pub const UART10: usize = 29;
}
pub mod pins {
    pub const PA00: usize = 0;
    pub const PA01: usize = 1;
    pub const PA02: usize = 2;
    pub const PA03: usize = 3;
    pub const PA04: usize = 4;
    pub const PA05: usize = 5;
    pub const PA06: usize = 6;
    pub const PA07: usize = 7;
    pub const PA08: usize = 8;
    pub const PA09: usize = 9;
    pub const PA10: usize = 10;
    pub const PA11: usize = 11;
    pub const PA12: usize = 12;
    pub const PA13: usize = 13;
    pub const PA14: usize = 14;
    pub const PA15: usize = 15;
    pub const PA16: usize = 16;
    pub const PA17: usize = 17;
    pub const PA18: usize = 18;
    pub const PA19: usize = 19;
    pub const PA20: usize = 20;
    pub const PA21: usize = 21;
    pub const PA22: usize = 22;
    pub const PA23: usize = 23;
    pub const PA24: usize = 24;
    pub const PA25: usize = 25;
    pub const PA26: usize = 26;
    pub const PA27: usize = 27;
    pub const PA28: usize = 28;
    pub const PA29: usize = 29;
    pub const PA30: usize = 30;
    pub const PA31: usize = 31;
    pub const PB00: usize = 32;
    pub const PB01: usize = 33;
    pub const PB02: usize = 34;
    pub const PB03: usize = 35;
    pub const PB04: usize = 36;
    pub const PB05: usize = 37;
    pub const PB06: usize = 38;
    pub const PB07: usize = 39;
    pub const PB08: usize = 40;
    pub const PB09: usize = 41;
    pub const PB10: usize = 42;
    pub const PB11: usize = 43;
    pub const PB12: usize = 44;
    pub const PB13: usize = 45;
    pub const PB14: usize = 46;
    pub const PB15: usize = 47;
    pub const PB16: usize = 48;
    pub const PB17: usize = 49;
    pub const PB18: usize = 50;
    pub const PB19: usize = 51;
    pub const PB20: usize = 52;
    pub const PB21: usize = 53;
    pub const PB22: usize = 54;
    pub const PB23: usize = 55;
    pub const PB24: usize = 56;
    pub const PB25: usize = 57;
    pub const PB26: usize = 58;
    pub const PB27: usize = 59;
    pub const PB28: usize = 60;
    pub const PB29: usize = 61;
    pub const PB30: usize = 62;
    pub const PB31: usize = 63;
    pub const PC00: usize = 64;
    pub const PC01: usize = 65;
    pub const PC02: usize = 66;
    pub const PC03: usize = 67;
    pub const PC04: usize = 68;
    pub const PC05: usize = 69;
    pub const PC06: usize = 70;
    pub const PC07: usize = 71;
    pub const PC08: usize = 72;
    pub const PC09: usize = 73;
    pub const PC10: usize = 74;
    pub const PC11: usize = 75;
    pub const PC12: usize = 76;
    pub const PC13: usize = 77;
    pub const PC14: usize = 78;
    pub const PC15: usize = 79;
    pub const PC16: usize = 80;
    pub const PC17: usize = 81;
    pub const PC18: usize = 82;
    pub const PC19: usize = 83;
    pub const PC20: usize = 84;
    pub const PC21: usize = 85;
    pub const PC22: usize = 86;
    pub const PC23: usize = 87;
    pub const PC24: usize = 88;
    pub const PC25: usize = 89;
    pub const PC26: usize = 90;
    pub const PC27: usize = 91;
    pub const PC28: usize = 92;
    pub const PC29: usize = 93;
    pub const PC30: usize = 94;
    pub const PC31: usize = 95;
    pub const PD00: usize = 96;
    pub const PD01: usize = 97;
    pub const PD02: usize = 98;
    pub const PD03: usize = 99;
    pub const PD04: usize = 100;
    pub const PD05: usize = 101;
    pub const PD06: usize = 102;
    pub const PD07: usize = 103;
    pub const PD08: usize = 104;
    pub const PD09: usize = 105;
    pub const PD10: usize = 106;
    pub const PD11: usize = 107;
    pub const PD12: usize = 108;
    pub const PD13: usize = 109;
    pub const PD14: usize = 110;
    pub const PD15: usize = 111;
    pub const PD16: usize = 112;
    pub const PD17: usize = 113;
    pub const PD18: usize = 114;
    pub const PD19: usize = 115;
    pub const PD20: usize = 116;
    pub const PD21: usize = 117;
    pub const PD22: usize = 118;
    pub const PD23: usize = 119;
    pub const PD24: usize = 120;
    pub const PD25: usize = 121;
    pub const PD26: usize = 122;
    pub const PD27: usize = 123;
    pub const PD28: usize = 124;
    pub const PD29: usize = 125;
    pub const PD30: usize = 126;
    pub const PD31: usize = 127;
    pub const PE00: usize = 128;
    pub const PE01: usize = 129;
    pub const PE02: usize = 130;
    pub const PE03: usize = 131;
    pub const PE04: usize = 132;
    pub const PE05: usize = 133;
    pub const PE06: usize = 134;
    pub const PE07: usize = 135;
    pub const PE08: usize = 136;
    pub const PE09: usize = 137;
    pub const PE10: usize = 138;
    pub const PE11: usize = 139;
    pub const PE12: usize = 140;
    pub const PE13: usize = 141;
    pub const PE14: usize = 142;
    pub const PE15: usize = 143;
    pub const PE16: usize = 144;
    pub const PE17: usize = 145;
    pub const PE18: usize = 146;
    pub const PE19: usize = 147;
    pub const PE20: usize = 148;
    pub const PE21: usize = 149;
    pub const PE22: usize = 150;
    pub const PE23: usize = 151;
    pub const PE24: usize = 152;
    pub const PE25: usize = 153;
    pub const PE26: usize = 154;
    pub const PE27: usize = 155;
    pub const PE28: usize = 156;
    pub const PE29: usize = 157;
    pub const PE30: usize = 158;
    pub const PE31: usize = 159;
    pub const PF00: usize = 160;
    pub const PF01: usize = 161;
    pub const PF02: usize = 162;
    pub const PF03: usize = 163;
    pub const PF04: usize = 164;
    pub const PF05: usize = 165;
    pub const PF06: usize = 166;
    pub const PF07: usize = 167;
    pub const PF08: usize = 168;
    pub const PF09: usize = 169;
    pub const PF10: usize = 170;
    pub const PX00: usize = 416;
    pub const PX01: usize = 417;
    pub const PX02: usize = 418;
    pub const PX03: usize = 419;
    pub const PX04: usize = 420;
    pub const PX05: usize = 421;
    pub const PX06: usize = 422;
    pub const PX07: usize = 423;
    pub const PX08: usize = 424;
    pub const PX09: usize = 425;
    pub const PX10: usize = 426;
    pub const PX11: usize = 427;
    pub const PY00: usize = 448;
    pub const PY01: usize = 449;
    pub const PY02: usize = 450;
    pub const PY03: usize = 451;
    pub const PY04: usize = 452;
    pub const PY05: usize = 453;
    pub const PY06: usize = 454;
    pub const PY07: usize = 455;
    pub const PY08: usize = 456;
    pub const PY09: usize = 457;
    pub const PY10: usize = 458;
    pub const PY11: usize = 459;
    pub const PZ00: usize = 480;
    pub const PZ01: usize = 481;
    pub const PZ02: usize = 482;
    pub const PZ03: usize = 483;
    pub const PZ04: usize = 484;
    pub const PZ05: usize = 485;
    pub const PZ06: usize = 486;
    pub const PZ07: usize = 487;
    pub const PZ08: usize = 488;
    pub const PZ09: usize = 489;
    pub const PZ10: usize = 490;
    pub const PZ11: usize = 491;
}
