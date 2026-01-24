include!("../metadata_0013.rs");
pub static METADATA: Metadata = Metadata {
    name: "HPM6330",
    family: "HPM6300 Series",
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
            name: "AXI_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x1080000,
            size: 524288,
        },
        MemoryRegion {
            name: "AHB_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0xf0300000,
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
