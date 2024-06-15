include!("../metadata_0026.rs");
pub static METADATA: Metadata = Metadata {
    name: "HPM6850",
    family: "HPM6800 Series",
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
            address: 0x1200000,
            size: 524288,
        },
        MemoryRegion {
            name: "AHB_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0xf0400000,
            size: 32768,
        },
        MemoryRegion {
            name: "APB_SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0xf4130000,
            size: 16384,
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
};
