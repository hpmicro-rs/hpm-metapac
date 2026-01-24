include!("../metadata_0032.rs");
pub static METADATA: Metadata = Metadata {
    name: "HPM6E60",
    family: "HPM6E00 Series",
    memory: &[
        MemoryRegion {
            name: "ILM0",
            kind: MemoryRegionKind::Ram,
            address: 0x0,
            size: 262144,
        },
        MemoryRegion {
            name: "DLM0",
            kind: MemoryRegionKind::Ram,
            address: 0x200000,
            size: 262144,
        },
        MemoryRegion {
            name: "AXI_SRAM0",
            kind: MemoryRegionKind::Ram,
            address: 0x1200000,
            size: 524288,
        },
        MemoryRegion {
            name: "AXI_SRAM1",
            kind: MemoryRegionKind::Ram,
            address: 0x1280000,
            size: 524288,
        },
        MemoryRegion {
            name: "AHB_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0xf0200000,
            size: 32768,
        },
        MemoryRegion {
            name: "XPI0",
            kind: MemoryRegionKind::Flash,
            address: 0x80000000,
            size: 1048576,
        },
    ],
    peripherals: PERIPHERALS,
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    resources: RESOURCES,
    clocks: CLOCKS,
    pins: PINS,
    trgmmux: TRGMMUX,
};
