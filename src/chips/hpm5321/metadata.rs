include!("../metadata_0001.rs");
pub static METADATA: Metadata = Metadata {
    name: "HPM5321",
    family: "HPM5300 Series",
    memory: &[
        MemoryRegion {
            name: "ILM",
            kind: MemoryRegionKind::Ram,
            address: 0x0,
            size: 131072,
        },
        MemoryRegion {
            name: "DLM",
            kind: MemoryRegionKind::Ram,
            address: 0x80000,
            size: 131072,
        },
        MemoryRegion {
            name: "AHB_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0xf0400000,
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
