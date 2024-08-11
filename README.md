# hpm-data & hpm-metapac

[![CI Status][badge-actions]][actions-build]
[![Crates.io][badge-crates-io]][crates-io]
[![Docs.rs][badge-docs-rs]][docs-rs]

[badge-actions]: https://img.shields.io/github/actions/workflow/status/andelf/hpm-data/build.yml?style=for-the-badge&label=CI&20Tests
[actions-build]: https://github.com/andelf/hpm-data/actions/workflows/build.yml
[badge-crates-io]: https://img.shields.io/crates/v/hpm-metapac.svg?style=for-the-badge
[crates-io]: https://crates.io/crates/hpm-metapac
[badge-docs-rs]: https://img.shields.io/docsrs/hpm-metapac?style=for-the-badge
[docs-rs]: https://docs.rs/hpm-metapac

The structured MCU DB of HPM MCUs. The home of [hpm-metapac][docs-rs].

All PRs and Issues are handled in [andelf/hpm-data](https://github.com/andelf/hpm-data).

`hpm-metapac` is generated from this repo. For each commit(or push) of hpm-data, it's pushed to <https://github.com/hpmicro-rs/hpm-metapac>,
with a tag of `hpm-data-<commit-hash>`.

## hpm-metapac

- The `hpm-metapac` crate has a `metadata` feature, when enabled, it will provide the basic metadata of the currrent MCU
- Patch vectored interrupt mode, add `CORE_LOCAL` for Non-External Interrupts
- To best fit for HPM RISC-V's clustered register desigin, the following is added:
  - All clocks, for `SYSCTL.CLOCK`, under `hpm_metapac::clocks::`
  - All SYSCTL resources, under `hpm_metapac::resources::`
  - All GPIOs and it's PADs, for `IOC`, under `hpm_metapac::pins::`
  - All IOMUX settings (`FUNC_CTL`), under `hpm_metapac::iomux::`
  - All TRGM const definitions, under `hpm_metapac::trgmmux::`
- The version on crates.io is not updated frequently, please use the git repo directly

### Usage

```toml
[dependencies]
hpm-metapac = { version = "0.0.3", git = "https://github.com/hpmicro-rs/hpm-metapac.git", tag = "hpm-data-6740ca6fd1ed6d9bb57944b42aa299761b974713", features = ["hpm5361"] }

# If you want to use the metadata feature in build.rs
[build-dependencies]
hpm-metapac = { version = "0.0.3", git = "https://github.com/hpmicro-rs/hpm-metapac.git", tag = "hpm-data-6740ca6fd1ed6d9bb57944b42aa299761b974713", default-features = false, features = [
    "metadata",
    "hpm5361",
] }
```

A simple example to configure pin PA25 for PWM1_P1:

```rust
use hpm_metapac as pac;
use pac::{iomux, pins};

pac::IOC
    .pad(pins::PA25)
    .func_ctl()
    .modify(|w| w.set_alt_select(iomux::IOC_PA25_FUNC_CTL_PWM1_P_1));
```

## Support Status

- All peripherals are supported
- All MCU families are supported
- Peripherals that have a HAL driver or raw PAC demo in [hpm-hal](https://github.com/hpmicro-rs/hpm-hal) is reviewed and tested

### MCU Family

(in order of release date)

- HPM6700/HPM6400 - high performance
- HPM6300 - general purpose
- HPM6200 - high performance, real-time, mixed signal
- HPM5300 - general purpose, motion control
- HPM6800 - display, user interface
- HPM6E00 - EtherCAT

- [x] HPM6700/HPM6400
- [x] HPM6300
- [x] HPM6200
- [x] HPM5300
- [x] HPM6800
- [x] HPM6E00

## Data Source

- <https://www.hpmicro.com/>
- <https://github.com/hpmicro/hpm_pinmux_tool>
- <https://github.com/hpmicro/hpm_sdk>
- <https://tools.hpmicro.com/pinmux>
