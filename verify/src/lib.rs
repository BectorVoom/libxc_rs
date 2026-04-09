//! Verification harness for libxc_rs.
//!
//! This crate links against the vendored C libxc 7.0.0 library via FFI
//! and provides oracle functions for comparing Rust implementations against
//! the reference C library.

pub mod oracle_ffi;

use anyhow::{bail, ensure, Result};

/// Call C libxc to evaluate LDA exchange-correlation energy density.
///
/// # Arguments
/// * `func_id` - Functional ID (e.g., 1 for LDA_X)
/// * `spin` - Spin mode: 1 = unpolarized, 2 = polarized
/// * `rho` - Density values. For unpolarized: one value per grid point.
///           For polarized: interleaved [rho_up, rho_down] pairs.
///
/// # Returns
/// Vec of energy density values, one per grid point.
pub fn oracle_lda_exc(func_id: i32, spin: i32, rho: &[f64]) -> Result<Vec<f64>> {
    ensure!(
        spin == 1 || spin == 2,
        "spin must be 1 (unpolarized) or 2 (polarized), got {spin}"
    );

    let np = if spin == 1 {
        rho.len()
    } else {
        ensure!(
            rho.len() % 2 == 0,
            "polarized rho must have even length, got {}",
            rho.len()
        );
        rho.len() / 2
    };

    ensure!(np > 0, "rho must not be empty");

    let mut exc = vec![0.0f64; np];

    unsafe {
        let func = oracle_ffi::xc_func_alloc();
        if func.is_null() {
            bail!("xc_func_alloc returned null");
        }

        let ret = oracle_ffi::xc_func_init(func, func_id, spin);
        if ret != 0 {
            oracle_ffi::xc_func_free(func);
            bail!("xc_func_init failed with code {ret} for func_id={func_id}, spin={spin}");
        }

        oracle_ffi::xc_lda_exc(func, np, rho.as_ptr(), exc.as_mut_ptr());

        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
    }

    Ok(exc)
}
