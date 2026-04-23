#![allow(unused, clippy::needless_range_loop)]

pub mod core {
    pub mod data_types;
    pub mod errors;
    pub mod instances;
    pub mod records;
}

pub mod io {
    pub mod file;
    pub mod helpers;

    #[cfg(feature = "dom")]
    pub mod writer;
}

pub use core::{data_types, errors, instances, records};
pub use io::file;

#[cfg(feature = "dom")]
pub mod dom;

#[cfg(feature = "prelude")]
pub mod prelude;
