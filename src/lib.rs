//! Peripheral Access Crate (PAC) for all HPMicro chips, including metadata.
#![no_std]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(non_camel_case_types)]
#![doc(html_no_source)]

#[cfg(feature = "pac")]
include!(env!("HPM_METAPAC_PAC_PATH"));

#[cfg(feature = "metadata")]
pub mod metadata {
    include!("metadata.rs");
    include!(env!("HPM_METAPAC_METADATA_PATH"));
}
