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
///   For polarized: interleaved \[rho_up, rho_down\] pairs.
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
            rho.len().is_multiple_of(2),
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

/// Oracle output for LDA evaluation including all derivative orders.
pub struct LdaOracleOutput {
    pub zk: Vec<f64>,
    pub vrho: Vec<f64>,
    pub v2rho2: Vec<f64>,
    pub v3rho3: Vec<f64>,
    pub v4rho4: Vec<f64>,
}

/// Call C libxc to evaluate LDA with all derivatives up to 4th order.
///
/// # Arguments
/// * `func_id` - Functional ID (e.g., 1 for LDA_X)
/// * `spin` - Spin mode: 1 = unpolarized, 2 = polarized
/// * `rho` - Density values
///
/// # Returns
/// All derivative outputs through 4th order.
pub fn oracle_lda_all(func_id: i32, spin: i32, rho: &[f64]) -> Result<LdaOracleOutput> {
    ensure!(
        spin == 1 || spin == 2,
        "spin must be 1 (unpolarized) or 2 (polarized), got {spin}"
    );

    let np = if spin == 1 {
        rho.len()
    } else {
        ensure!(
            rho.len().is_multiple_of(2),
            "polarized rho must have even length, got {}",
            rho.len()
        );
        rho.len() / 2
    };

    ensure!(np > 0, "rho must not be empty");

    // Dimension multipliers per spin mode
    let (dim_vrho, dim_v2rho2, dim_v3rho3, dim_v4rho4) = if spin == 1 {
        (1, 1, 1, 1) // unpolarized: 1 component each
    } else {
        (2, 3, 4, 5) // polarized: 2, 3, 4, 5 components
    };

    let mut zk = vec![0.0f64; np];
    let mut vrho = vec![0.0f64; np * dim_vrho];
    let mut v2rho2 = vec![0.0f64; np * dim_v2rho2];
    let mut v3rho3 = vec![0.0f64; np * dim_v3rho3];
    let mut v4rho4 = vec![0.0f64; np * dim_v4rho4];

    unsafe {
        let func = oracle_ffi::xc_func_alloc();
        if func.is_null() {
            bail!("xc_func_alloc returned null");
        }

        let ret = oracle_ffi::xc_func_init(func, func_id, spin);
        if ret != 0 {
            oracle_ffi::xc_func_free(func);
            bail!("xc_func_init failed with code {ret}");
        }

        // Call exc+vxc+fxc+kxc together
        oracle_ffi::xc_lda_exc_vxc_fxc_kxc(
            func,
            np,
            rho.as_ptr(),
            zk.as_mut_ptr(),
            vrho.as_mut_ptr(),
            v2rho2.as_mut_ptr(),
            v3rho3.as_mut_ptr(),
        );

        // Call lxc separately (4th derivative)
        oracle_ffi::xc_lda_lxc(
            func,
            np,
            rho.as_ptr(),
            v4rho4.as_mut_ptr(),
        );

        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
    }

    Ok(LdaOracleOutput {
        zk,
        vrho,
        v2rho2,
        v3rho3,
        v4rho4,
    })
}

/// Options for configuring the oracle before evaluation.
#[derive(Default)]
pub struct OracleOptions {
    /// If Some, set the external parameters (e.g., alpha for LDA_X).
    pub ext_params: Option<Vec<f64>>,
    /// If Some, override the density threshold.
    pub dens_threshold: Option<f64>,
}

/// Call C libxc to evaluate LDA exc with custom options (ext_params, dens_threshold).
pub fn oracle_lda_exc_with_opts(
    func_id: i32,
    spin: i32,
    rho: &[f64],
    opts: &OracleOptions,
) -> Result<Vec<f64>> {
    ensure!(spin == 1 || spin == 2, "spin must be 1 or 2, got {spin}");

    let np = if spin == 1 {
        rho.len()
    } else {
        ensure!(rho.len().is_multiple_of(2), "polarized rho must have even length");
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
            bail!("xc_func_init failed with code {ret}");
        }

        if let Some(ref params) = opts.ext_params {
            oracle_ffi::xc_func_set_ext_params(func, params.as_ptr());
        }
        if let Some(threshold) = opts.dens_threshold {
            oracle_ffi::xc_func_set_dens_threshold(func, threshold);
        }

        oracle_ffi::xc_lda_exc(func, np, rho.as_ptr(), exc.as_mut_ptr());

        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
    }

    Ok(exc)
}

/// Call C libxc to evaluate LDA with all derivatives, with custom options.
pub fn oracle_lda_all_with_opts(
    func_id: i32,
    spin: i32,
    rho: &[f64],
    opts: &OracleOptions,
) -> Result<LdaOracleOutput> {
    ensure!(spin == 1 || spin == 2, "spin must be 1 or 2, got {spin}");

    let np = if spin == 1 {
        rho.len()
    } else {
        ensure!(rho.len().is_multiple_of(2), "polarized rho must have even length");
        rho.len() / 2
    };
    ensure!(np > 0, "rho must not be empty");

    let (dim_vrho, dim_v2rho2, dim_v3rho3, dim_v4rho4) = if spin == 1 {
        (1, 1, 1, 1)
    } else {
        (2, 3, 4, 5)
    };

    let mut zk = vec![0.0f64; np];
    let mut vrho = vec![0.0f64; np * dim_vrho];
    let mut v2rho2 = vec![0.0f64; np * dim_v2rho2];
    let mut v3rho3 = vec![0.0f64; np * dim_v3rho3];
    let mut v4rho4 = vec![0.0f64; np * dim_v4rho4];

    unsafe {
        let func = oracle_ffi::xc_func_alloc();
        if func.is_null() {
            bail!("xc_func_alloc returned null");
        }

        let ret = oracle_ffi::xc_func_init(func, func_id, spin);
        if ret != 0 {
            oracle_ffi::xc_func_free(func);
            bail!("xc_func_init failed with code {ret}");
        }

        if let Some(ref params) = opts.ext_params {
            oracle_ffi::xc_func_set_ext_params(func, params.as_ptr());
        }
        if let Some(threshold) = opts.dens_threshold {
            oracle_ffi::xc_func_set_dens_threshold(func, threshold);
        }

        oracle_ffi::xc_lda_exc_vxc_fxc_kxc(
            func,
            np,
            rho.as_ptr(),
            zk.as_mut_ptr(),
            vrho.as_mut_ptr(),
            v2rho2.as_mut_ptr(),
            v3rho3.as_mut_ptr(),
        );

        oracle_ffi::xc_lda_lxc(func, np, rho.as_ptr(), v4rho4.as_mut_ptr());

        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
    }

    Ok(LdaOracleOutput {
        zk,
        vrho,
        v2rho2,
        v3rho3,
        v4rho4,
    })
}
