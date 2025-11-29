include!("../metadata_0011.rs");
pub static METADATA: Metadata = Metadata {
    name: "HPM64A0",
    family: "HPM6400 Series",
    memory: &[
        MemoryRegion {
            name: "ILM",
            kind: MemoryRegionKind::Ram,
            address: 0x0,
            size: 262144,
        },
        MemoryRegion {
            name: "DLM",
            kind: MemoryRegionKind::Ram,
            address: 0x80000,
            size: 262144,
        },
        MemoryRegion {
            name: "AXI_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x1080000,
            size: 524288,
        },
        MemoryRegion {
            name: "AXI_SRAM_NOCACHE",
            kind: MemoryRegionKind::Ram,
            address: 0x1100000,
            size: 262144,
        },
        MemoryRegion {
            name: "SHARE_RAM",
            kind: MemoryRegionKind::Ram,
            address: 0x117c000,
            size: 16384,
        },
        MemoryRegion {
            name: "AHB_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0xf0300000,
            size: 32768,
        },
        MemoryRegion {
            name: "APB_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0xf40f0000,
            size: 8192,
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
