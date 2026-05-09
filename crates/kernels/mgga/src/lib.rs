#![allow(clippy::excessive_precision)]
#![allow(clippy::needless_late_init)]
#![allow(clippy::too_many_arguments)]

//! MGGA kernel translations from maple2c.
//!
//! 92 MGGA functionals total across 37 sub-crates.
//! Each sub-crate is sized via first-fit-decreasing bin packing to stay under
//! ~50K lines of generated Rust, avoiding OOM during CubeCL proc macro expansion.
//! Seven large functionals (62K-86K lines) occupy solo crates since they exceed
//! 50K even as single modules.

// Re-export sub-crates containing compiled MGGA functionals.
pub use libxc_kernel_mgga_1 as batch1;
pub use libxc_kernel_mgga_2 as batch2;
pub use libxc_kernel_mgga_3 as batch3;
pub use libxc_kernel_mgga_4 as batch4;
pub use libxc_kernel_mgga_5 as batch5;
pub use libxc_kernel_mgga_6 as batch6;
pub use libxc_kernel_mgga_7 as batch7;
pub use libxc_kernel_mgga_8a as batch8a;
pub use libxc_kernel_mgga_8b as batch8b;
pub use libxc_kernel_mgga_9a as batch9a;
pub use libxc_kernel_mgga_9b as batch9b;
pub use libxc_kernel_mgga_10 as batch10;
pub use libxc_kernel_mgga_11a as batch11a;
pub use libxc_kernel_mgga_11b as batch11b;
pub use libxc_kernel_mgga_12 as batch12;
pub use libxc_kernel_mgga_13 as batch13;
pub use libxc_kernel_mgga_14 as batch14;
