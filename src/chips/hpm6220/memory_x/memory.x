MEMORY
{
    XPI0  : ORIGIN = 0x80000000, LENGTH = 1024K /* bootheader and firmware */
    DLM   : ORIGIN = 0x00080000, LENGTH =  128K /* data local memory */
    ILM   : ORIGIN = 0x00000000, LENGTH =  128K /* instruction local memory */
    AXI_SRAM   : ORIGIN = 0x010a0000, LENGTH =  112K
    AHB_SRAM   : ORIGIN = 0xf0300000, LENGTH =   32K
}
REGION_ALIAS("REGION_TEXT", XPI0);
REGION_ALIAS("REGION_RODATA", XPI0);
REGION_ALIAS("REGION_DATA", DLM);
REGION_ALIAS("REGION_BSS", DLM);
REGION_ALIAS("REGION_HEAP", DLM);
REGION_ALIAS("REGION_STACK", DLM);
    