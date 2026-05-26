//! Kernel re-export façade.
//!
//! Phase 11 D-10b: the per-family façade crates (`libxc_kernel_lda`,
//! `libxc_kernel_gga`, `libxc_kernel_mgga`) were deleted by D-10a's
//! clean-slate restructure. `lda` / `gga` / `mgga` are now generated modules
//! (`src/kernel/{lda,gga,mgga}.rs`) that re-export every per-functional kernel
//! subcrate under the family namespace, so `crate::kernel::<family>::<func>`
//! still resolves. Regenerate them with `tools/generate_kernel_reexports.py`.

// Phase 11 / 11-12 (G-2, Path A): family kernel re-exports are gated behind
// per-family features so a `--no-default-features -F oracle-<fam>` build pulls
// (and references) only that family's kernels. default = all three, so the full
// umbrella build is unchanged.
#[cfg(feature = "oracle-lda")]
pub mod lda;
#[cfg(feature = "oracle-gga")]
pub mod gga;
#[cfg(feature = "oracle-mgga")]
pub mod mgga;
pub mod launch;
pub mod dispatch_key;
pub mod shared;
pub mod mix;
