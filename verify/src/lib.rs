//! Verification harness for libxc_rs.
//!
//! This crate links against the vendored C libxc 7.0.0 library via FFI
//! and provides oracle functions for comparing Rust implementations against
//! the reference C library.

pub mod oracle_ffi;

use anyhow::{bail, ensure, Result};

/// Flags indicating which derivative levels a functional supports.
pub const FLAGS_HAVE_EXC: i32 = 1 << 0; //     1
pub const FLAGS_HAVE_VXC: i32 = 1 << 1; //     2
pub const FLAGS_HAVE_FXC: i32 = 1 << 2; //     4
pub const FLAGS_HAVE_KXC: i32 = 1 << 3; //     8
pub const FLAGS_HAVE_LXC: i32 = 1 << 4; //    16

/// Query the capability flags of a libxc functional.
/// Returns None if the functional cannot be initialized.
pub fn oracle_func_flags(func_id: i32, spin: i32) -> Option<i32> {
    unsafe {
        let func = oracle_ffi::xc_func_alloc();
        if func.is_null() {
            return None;
        }
        let ret = oracle_ffi::xc_func_init(func, func_id, spin);
        if ret != 0 {
            oracle_ffi::xc_func_free(func);
            return None;
        }
        let info = (*func).info;
        let flags = if info.is_null() { 0 } else { (*info).flags as i32 };
        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
        Some(flags)
    }
}

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

// ============================================================================
// GGA Oracle Functions
// ============================================================================

/// Oracle output for GGA evaluation including all derivative orders.
pub struct GgaOracleOutput {
    pub zk: Vec<f64>,
    pub vrho: Vec<f64>,
    pub vsigma: Vec<f64>,
    pub v2rho2: Vec<f64>,
    pub v2rhosigma: Vec<f64>,
    pub v2sigma2: Vec<f64>,
    pub v3rho3: Vec<f64>,
    pub v3rho2sigma: Vec<f64>,
    pub v3rhosigma2: Vec<f64>,
    pub v3sigma3: Vec<f64>,
    pub v4rho4: Vec<f64>,
    pub v4rho3sigma: Vec<f64>,
    pub v4rho2sigma2: Vec<f64>,
    pub v4rhosigma3: Vec<f64>,
    pub v4sigma4: Vec<f64>,
}

/// Call C libxc to evaluate GGA with all derivatives up to 4th order.
pub fn oracle_gga_all(func_id: i32, spin: i32, rho: &[f64], sigma: &[f64]) -> Result<GgaOracleOutput> {
    oracle_gga_all_with_opts(func_id, spin, rho, sigma, &OracleOptions::default())
}

/// Call C libxc to evaluate GGA with all derivatives, with custom options.
pub fn oracle_gga_all_with_opts(
    func_id: i32,
    spin: i32,
    rho: &[f64],
    sigma: &[f64],
    opts: &OracleOptions,
) -> Result<GgaOracleOutput> {
    ensure!(spin == 1 || spin == 2, "spin must be 1 or 2, got {spin}");

    let np = if spin == 1 {
        rho.len()
    } else {
        ensure!(rho.len().is_multiple_of(2), "polarized rho must have even length");
        rho.len() / 2
    };
    ensure!(np > 0, "rho must not be empty");

    // GGA dimension multipliers per spin mode (from libxc util.c internal_counters_set_gga)
    // LDA base: vrho=nspin, v2rho2=3, v3rho3=4, v4rho4=5
    // GGA adds: sigma=3, vsigma=3, v2rhosigma=2*3=6, v2sigma2=6
    //           v3rho2sigma=3*3=9, v3rhosigma2=2*6=12, v3sigma3=10
    //           v4rho3sigma=4*3=12, v4rho2sigma2=3*6=18, v4rhosigma3=2*10=20, v4sigma4=15
    let (d_vrho, d_vsigma, d_v2rho2, d_v2rhosigma, d_v2sigma2,
         d_v3rho3, d_v3rho2sigma, d_v3rhosigma2, d_v3sigma3,
         d_v4rho4, d_v4rho3sigma, d_v4rho2sigma2, d_v4rhosigma3, d_v4sigma4) = if spin == 1 {
        (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)
    } else {
        (2, 3, 3, 6, 6, 4, 9, 12, 10, 5, 12, 18, 20, 15)
    };

    let mut zk = vec![0.0f64; np];
    let mut vrho = vec![0.0f64; np * d_vrho];
    let mut vsigma = vec![0.0f64; np * d_vsigma];
    let mut v2rho2 = vec![0.0f64; np * d_v2rho2];
    let mut v2rhosigma = vec![0.0f64; np * d_v2rhosigma];
    let mut v2sigma2 = vec![0.0f64; np * d_v2sigma2];
    let mut v3rho3 = vec![0.0f64; np * d_v3rho3];
    let mut v3rho2sigma = vec![0.0f64; np * d_v3rho2sigma];
    let mut v3rhosigma2 = vec![0.0f64; np * d_v3rhosigma2];
    let mut v3sigma3 = vec![0.0f64; np * d_v3sigma3];
    let mut v4rho4 = vec![0.0f64; np * d_v4rho4];
    let mut v4rho3sigma = vec![0.0f64; np * d_v4rho3sigma];
    let mut v4rho2sigma2 = vec![0.0f64; np * d_v4rho2sigma2];
    let mut v4rhosigma3 = vec![0.0f64; np * d_v4rhosigma3];
    let mut v4sigma4 = vec![0.0f64; np * d_v4sigma4];

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

        if let Some(ref params) = opts.ext_params {
            oracle_ffi::xc_func_set_ext_params(func, params.as_ptr());
        }
        if let Some(threshold) = opts.dens_threshold {
            oracle_ffi::xc_func_set_dens_threshold(func, threshold);
        }

        // Check which derivative levels are supported
        let info = (*func).info;
        let flags = if info.is_null() { 0 } else { (*info).flags as i32 };

        let has_fxc = flags & FLAGS_HAVE_FXC != 0;
        let has_kxc = flags & FLAGS_HAVE_KXC != 0;
        let has_lxc = flags & FLAGS_HAVE_LXC != 0;

        // Use null pointers for unsupported derivative levels to avoid libxc exit()
        let v2rho2_ptr = if has_fxc { v2rho2.as_mut_ptr() } else { std::ptr::null_mut() };
        let v2rhosigma_ptr = if has_fxc { v2rhosigma.as_mut_ptr() } else { std::ptr::null_mut() };
        let v2sigma2_ptr = if has_fxc { v2sigma2.as_mut_ptr() } else { std::ptr::null_mut() };
        let v3rho3_ptr = if has_kxc { v3rho3.as_mut_ptr() } else { std::ptr::null_mut() };
        let v3rho2sigma_ptr = if has_kxc { v3rho2sigma.as_mut_ptr() } else { std::ptr::null_mut() };
        let v3rhosigma2_ptr = if has_kxc { v3rhosigma2.as_mut_ptr() } else { std::ptr::null_mut() };
        let v3sigma3_ptr = if has_kxc { v3sigma3.as_mut_ptr() } else { std::ptr::null_mut() };

        oracle_ffi::xc_gga_exc_vxc_fxc_kxc(
            func, np, rho.as_ptr(), sigma.as_ptr(),
            zk.as_mut_ptr(), vrho.as_mut_ptr(), vsigma.as_mut_ptr(),
            v2rho2_ptr, v2rhosigma_ptr, v2sigma2_ptr,
            v3rho3_ptr, v3rho2sigma_ptr, v3rhosigma2_ptr, v3sigma3_ptr,
        );

        if has_lxc {
            oracle_ffi::xc_gga_lxc(
                func, np, rho.as_ptr(), sigma.as_ptr(),
                v4rho4.as_mut_ptr(), v4rho3sigma.as_mut_ptr(), v4rho2sigma2.as_mut_ptr(),
                v4rhosigma3.as_mut_ptr(), v4sigma4.as_mut_ptr(),
            );
        }

        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
    }

    Ok(GgaOracleOutput {
        zk, vrho, vsigma,
        v2rho2, v2rhosigma, v2sigma2,
        v3rho3, v3rho2sigma, v3rhosigma2, v3sigma3,
        v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3, v4sigma4,
    })
}

// ============================================================================
// MGGA Oracle Functions
// ============================================================================

/// Oracle output for MGGA evaluation including all derivative orders.
#[allow(dead_code)]
pub struct MggaOracleOutput {
    // 0th order
    pub zk: Vec<f64>,
    // 1st order
    pub vrho: Vec<f64>,
    pub vsigma: Vec<f64>,
    pub vlapl: Vec<f64>,
    pub vtau: Vec<f64>,
    // 2nd order
    pub v2rho2: Vec<f64>,
    pub v2rhosigma: Vec<f64>,
    pub v2rholapl: Vec<f64>,
    pub v2rhotau: Vec<f64>,
    pub v2sigma2: Vec<f64>,
    pub v2sigmalapl: Vec<f64>,
    pub v2sigmatau: Vec<f64>,
    pub v2lapl2: Vec<f64>,
    pub v2lapltau: Vec<f64>,
    pub v2tau2: Vec<f64>,
    // 3rd order
    pub v3rho3: Vec<f64>,
    pub v3rho2sigma: Vec<f64>,
    pub v3rho2lapl: Vec<f64>,
    pub v3rho2tau: Vec<f64>,
    pub v3rhosigma2: Vec<f64>,
    pub v3rhosigmalapl: Vec<f64>,
    pub v3rhosigmatau: Vec<f64>,
    pub v3rholapl2: Vec<f64>,
    pub v3rholapltau: Vec<f64>,
    pub v3rhotau2: Vec<f64>,
    pub v3sigma3: Vec<f64>,
    pub v3sigma2lapl: Vec<f64>,
    pub v3sigma2tau: Vec<f64>,
    pub v3sigmalapl2: Vec<f64>,
    pub v3sigmalapltau: Vec<f64>,
    pub v3sigmatau2: Vec<f64>,
    pub v3lapl3: Vec<f64>,
    pub v3lapl2tau: Vec<f64>,
    pub v3lapltau2: Vec<f64>,
    pub v3tau3: Vec<f64>,
    // 4th order
    pub v4rho4: Vec<f64>,
    pub v4rho3sigma: Vec<f64>,
    pub v4rho3lapl: Vec<f64>,
    pub v4rho3tau: Vec<f64>,
    pub v4rho2sigma2: Vec<f64>,
    pub v4rho2sigmalapl: Vec<f64>,
    pub v4rho2sigmatau: Vec<f64>,
    pub v4rho2lapl2: Vec<f64>,
    pub v4rho2lapltau: Vec<f64>,
    pub v4rho2tau2: Vec<f64>,
    pub v4rhosigma3: Vec<f64>,
    pub v4rhosigma2lapl: Vec<f64>,
    pub v4rhosigma2tau: Vec<f64>,
    pub v4rhosigmalapl2: Vec<f64>,
    pub v4rhosigmalapltau: Vec<f64>,
    pub v4rhosigmatau2: Vec<f64>,
    pub v4rholapl3: Vec<f64>,
    pub v4rholapl2tau: Vec<f64>,
    pub v4rholapltau2: Vec<f64>,
    pub v4rhotau3: Vec<f64>,
    pub v4sigma4: Vec<f64>,
    pub v4sigma3lapl: Vec<f64>,
    pub v4sigma3tau: Vec<f64>,
    pub v4sigma2lapl2: Vec<f64>,
    pub v4sigma2lapltau: Vec<f64>,
    pub v4sigma2tau2: Vec<f64>,
    pub v4sigmalapl3: Vec<f64>,
    pub v4sigmalapl2tau: Vec<f64>,
    pub v4sigmalapltau2: Vec<f64>,
    pub v4sigmatau3: Vec<f64>,
    pub v4lapl4: Vec<f64>,
    pub v4lapl3tau: Vec<f64>,
    pub v4lapl2tau2: Vec<f64>,
    pub v4lapltau3: Vec<f64>,
    pub v4tau4: Vec<f64>,
}

/// Call C libxc to evaluate MGGA with all derivatives up to 4th order.
pub fn oracle_mgga_all(
    func_id: i32,
    spin: i32,
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
) -> Result<MggaOracleOutput> {
    oracle_mgga_all_with_opts(func_id, spin, rho, sigma, lapl, tau, &OracleOptions::default())
}

/// Call C libxc to evaluate MGGA with all derivatives, with custom options.
pub fn oracle_mgga_all_with_opts(
    func_id: i32,
    spin: i32,
    rho: &[f64],
    sigma: &[f64],
    lapl: &[f64],
    tau: &[f64],
    opts: &OracleOptions,
) -> Result<MggaOracleOutput> {
    ensure!(spin == 1 || spin == 2, "spin must be 1 or 2, got {spin}");

    let np = if spin == 1 {
        rho.len()
    } else {
        ensure!(rho.len().is_multiple_of(2), "polarized rho must have even length");
        rho.len() / 2
    };
    ensure!(np > 0, "rho must not be empty");

    // MGGA dimension multipliers per spin mode (from libxc util.c)
    let (d_vrho, d_vsigma, d_vlapl, d_vtau) = if spin == 1 {
        (1, 1, 1, 1)
    } else {
        (2, 3, 2, 2)
    };

    let (d_v2rho2, d_v2rhosigma, d_v2rholapl, d_v2rhotau,
         d_v2sigma2, d_v2sigmalapl, d_v2sigmatau,
         d_v2lapl2, d_v2lapltau, d_v2tau2) = if spin == 1 {
        (1, 1, 1, 1, 1, 1, 1, 1, 1, 1)
    } else {
        (3, 6, 4, 4, 6, 6, 6, 3, 4, 3)
    };

    let (d_v3rho3, d_v3rho2sigma, d_v3rho2lapl, d_v3rho2tau,
         d_v3rhosigma2, d_v3rhosigmalapl, d_v3rhosigmatau,
         d_v3rholapl2, d_v3rholapltau, d_v3rhotau2,
         d_v3sigma3, d_v3sigma2lapl, d_v3sigma2tau,
         d_v3sigmalapl2, d_v3sigmalapltau, d_v3sigmatau2,
         d_v3lapl3, d_v3lapl2tau, d_v3lapltau2, d_v3tau3) = if spin == 1 {
        (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)
    } else {
        (4, 9, 6, 6, 12, 12, 12, 6, 8, 6, 10, 12, 12, 9, 12, 9, 4, 6, 6, 4)
    };

    // 4th order dimensions from libxc util.c internal_counters_set_mgga (polarized case)
    let (d_v4rho4, d_v4rho3sigma, d_v4rho3lapl, d_v4rho3tau,
         d_v4rho2sigma2, d_v4rho2sigmalapl, d_v4rho2sigmatau,
         d_v4rho2lapl2, d_v4rho2lapltau, d_v4rho2tau2,
         d_v4rhosigma3, d_v4rhosigma2lapl, d_v4rhosigma2tau,
         d_v4rhosigmalapl2, d_v4rhosigmalapltau, d_v4rhosigmatau2,
         d_v4rholapl3, d_v4rholapl2tau, d_v4rholapltau2, d_v4rhotau3,
         d_v4sigma4, d_v4sigma3lapl, d_v4sigma3tau,
         d_v4sigma2lapl2, d_v4sigma2lapltau, d_v4sigma2tau2,
         d_v4sigmalapl3, d_v4sigmalapl2tau, d_v4sigmalapltau2, d_v4sigmatau3,
         d_v4lapl4, d_v4lapl3tau, d_v4lapl2tau2, d_v4lapltau3, d_v4tau4) = if spin == 1 {
        (1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1)
    } else {
        // From util.c: GGA base + MGGA additions
        // v4rho4=5, v4rho3sigma=12 (4*3), v4rho3lapl=8 (4*2), v4rho3tau=8 (4*2)
        // v4rho2sigma2=18 (3*6), v4rho2sigmalapl=18 (3*3*2), v4rho2sigmatau=18 (3*3*2)
        // v4rho2lapl2=9 (3*3), v4rho2lapltau=12 (3*2*2), v4rho2tau2=9 (3*3)
        // v4rhosigma3=20 (2*10), v4rhosigma2lapl=36 (3*6*2), v4rhosigma2tau=36 (3*6*2)
        // v4rhosigmalapl2=18 (2*3*3), v4rhosigmalapltau=24 (2*3*2*2), v4rhosigmatau2=36 (2*6*3)
        // v4rholapl3=8 (2*4), v4rholapl2tau=12 (2*3*2), v4rholapltau2=12 (2*2*3), v4rhotau3=8 (2*4)
        // v4sigma4=15, v4sigma3lapl=20 (10*2), v4sigma3tau=30 (10*3)
        // v4sigma2lapl2=18 (6*3), v4sigma2lapltau=24 (6*2*2), v4sigma2tau2=18 (6*3)
        // v4sigmalapl3=12 (3*4), v4sigmalapl2tau=18 (3*3*2), v4sigmalapltau2=18 (3*2*3), v4sigmatau3=12 (3*4)
        // v4lapl4=5, v4lapl3tau=8 (4*2), v4lapl2tau2=9 (3*3), v4lapltau3=8 (2*4), v4tau4=5
        (5, 12, 8, 8, 18, 18, 18, 9, 12, 9, 20, 36, 36, 18, 24, 36, 8, 12, 12, 8,
         15, 20, 30, 18, 24, 18, 12, 18, 18, 12, 5, 8, 9, 8, 5)
    };

    // Allocate all output buffers
    let mut zk = vec![0.0f64; np];
    let mut vrho = vec![0.0f64; np * d_vrho];
    let mut vsigma = vec![0.0f64; np * d_vsigma];
    let mut vlapl = vec![0.0f64; np * d_vlapl];
    let mut vtau = vec![0.0f64; np * d_vtau];

    let mut v2rho2 = vec![0.0f64; np * d_v2rho2];
    let mut v2rhosigma = vec![0.0f64; np * d_v2rhosigma];
    let mut v2rholapl = vec![0.0f64; np * d_v2rholapl];
    let mut v2rhotau = vec![0.0f64; np * d_v2rhotau];
    let mut v2sigma2 = vec![0.0f64; np * d_v2sigma2];
    let mut v2sigmalapl = vec![0.0f64; np * d_v2sigmalapl];
    let mut v2sigmatau = vec![0.0f64; np * d_v2sigmatau];
    let mut v2lapl2 = vec![0.0f64; np * d_v2lapl2];
    let mut v2lapltau = vec![0.0f64; np * d_v2lapltau];
    let mut v2tau2 = vec![0.0f64; np * d_v2tau2];

    let mut v3rho3 = vec![0.0f64; np * d_v3rho3];
    let mut v3rho2sigma = vec![0.0f64; np * d_v3rho2sigma];
    let mut v3rho2lapl = vec![0.0f64; np * d_v3rho2lapl];
    let mut v3rho2tau = vec![0.0f64; np * d_v3rho2tau];
    let mut v3rhosigma2 = vec![0.0f64; np * d_v3rhosigma2];
    let mut v3rhosigmalapl = vec![0.0f64; np * d_v3rhosigmalapl];
    let mut v3rhosigmatau = vec![0.0f64; np * d_v3rhosigmatau];
    let mut v3rholapl2 = vec![0.0f64; np * d_v3rholapl2];
    let mut v3rholapltau = vec![0.0f64; np * d_v3rholapltau];
    let mut v3rhotau2 = vec![0.0f64; np * d_v3rhotau2];
    let mut v3sigma3 = vec![0.0f64; np * d_v3sigma3];
    let mut v3sigma2lapl = vec![0.0f64; np * d_v3sigma2lapl];
    let mut v3sigma2tau = vec![0.0f64; np * d_v3sigma2tau];
    let mut v3sigmalapl2 = vec![0.0f64; np * d_v3sigmalapl2];
    let mut v3sigmalapltau = vec![0.0f64; np * d_v3sigmalapltau];
    let mut v3sigmatau2 = vec![0.0f64; np * d_v3sigmatau2];
    let mut v3lapl3 = vec![0.0f64; np * d_v3lapl3];
    let mut v3lapl2tau = vec![0.0f64; np * d_v3lapl2tau];
    let mut v3lapltau2 = vec![0.0f64; np * d_v3lapltau2];
    let mut v3tau3 = vec![0.0f64; np * d_v3tau3];

    let mut v4rho4 = vec![0.0f64; np * d_v4rho4];
    let mut v4rho3sigma = vec![0.0f64; np * d_v4rho3sigma];
    let mut v4rho3lapl = vec![0.0f64; np * d_v4rho3lapl];
    let mut v4rho3tau = vec![0.0f64; np * d_v4rho3tau];
    let mut v4rho2sigma2 = vec![0.0f64; np * d_v4rho2sigma2];
    let mut v4rho2sigmalapl = vec![0.0f64; np * d_v4rho2sigmalapl];
    let mut v4rho2sigmatau = vec![0.0f64; np * d_v4rho2sigmatau];
    let mut v4rho2lapl2 = vec![0.0f64; np * d_v4rho2lapl2];
    let mut v4rho2lapltau = vec![0.0f64; np * d_v4rho2lapltau];
    let mut v4rho2tau2 = vec![0.0f64; np * d_v4rho2tau2];
    let mut v4rhosigma3 = vec![0.0f64; np * d_v4rhosigma3];
    let mut v4rhosigma2lapl = vec![0.0f64; np * d_v4rhosigma2lapl];
    let mut v4rhosigma2tau = vec![0.0f64; np * d_v4rhosigma2tau];
    let mut v4rhosigmalapl2 = vec![0.0f64; np * d_v4rhosigmalapl2];
    let mut v4rhosigmalapltau = vec![0.0f64; np * d_v4rhosigmalapltau];
    let mut v4rhosigmatau2 = vec![0.0f64; np * d_v4rhosigmatau2];
    let mut v4rholapl3 = vec![0.0f64; np * d_v4rholapl3];
    let mut v4rholapl2tau = vec![0.0f64; np * d_v4rholapl2tau];
    let mut v4rholapltau2 = vec![0.0f64; np * d_v4rholapltau2];
    let mut v4rhotau3 = vec![0.0f64; np * d_v4rhotau3];
    let mut v4sigma4 = vec![0.0f64; np * d_v4sigma4];
    let mut v4sigma3lapl = vec![0.0f64; np * d_v4sigma3lapl];
    let mut v4sigma3tau = vec![0.0f64; np * d_v4sigma3tau];
    let mut v4sigma2lapl2 = vec![0.0f64; np * d_v4sigma2lapl2];
    let mut v4sigma2lapltau = vec![0.0f64; np * d_v4sigma2lapltau];
    let mut v4sigma2tau2 = vec![0.0f64; np * d_v4sigma2tau2];
    let mut v4sigmalapl3 = vec![0.0f64; np * d_v4sigmalapl3];
    let mut v4sigmalapl2tau = vec![0.0f64; np * d_v4sigmalapl2tau];
    let mut v4sigmalapltau2 = vec![0.0f64; np * d_v4sigmalapltau2];
    let mut v4sigmatau3 = vec![0.0f64; np * d_v4sigmatau3];
    let mut v4lapl4 = vec![0.0f64; np * d_v4lapl4];
    let mut v4lapl3tau = vec![0.0f64; np * d_v4lapl3tau];
    let mut v4lapl2tau2 = vec![0.0f64; np * d_v4lapl2tau2];
    let mut v4lapltau3 = vec![0.0f64; np * d_v4lapltau3];
    let mut v4tau4 = vec![0.0f64; np * d_v4tau4];

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

        if let Some(ref params) = opts.ext_params {
            oracle_ffi::xc_func_set_ext_params(func, params.as_ptr());
        }
        if let Some(threshold) = opts.dens_threshold {
            oracle_ffi::xc_func_set_dens_threshold(func, threshold);
        }

        // Check which derivative levels are supported
        let info = (*func).info;
        let flags = if info.is_null() { 0 } else { (*info).flags as i32 };

        let has_fxc = flags & FLAGS_HAVE_FXC != 0;
        let has_kxc = flags & FLAGS_HAVE_KXC != 0;

        // Use null pointers for unsupported derivative levels
        let np_ = |has: bool, p: &mut [f64]| -> *mut f64 {
            if has { p.as_mut_ptr() } else { std::ptr::null_mut() }
        };

        oracle_ffi::xc_mgga_exc_vxc_fxc_kxc(
            func, np,
            rho.as_ptr(), sigma.as_ptr(), lapl.as_ptr(), tau.as_ptr(),
            zk.as_mut_ptr(),
            vrho.as_mut_ptr(), vsigma.as_mut_ptr(), vlapl.as_mut_ptr(), vtau.as_mut_ptr(),
            np_(has_fxc, &mut v2rho2), np_(has_fxc, &mut v2rhosigma),
            np_(has_fxc, &mut v2rholapl), np_(has_fxc, &mut v2rhotau),
            np_(has_fxc, &mut v2sigma2), np_(has_fxc, &mut v2sigmalapl),
            np_(has_fxc, &mut v2sigmatau),
            np_(has_fxc, &mut v2lapl2), np_(has_fxc, &mut v2lapltau), np_(has_fxc, &mut v2tau2),
            np_(has_kxc, &mut v3rho3), np_(has_kxc, &mut v3rho2sigma),
            np_(has_kxc, &mut v3rho2lapl), np_(has_kxc, &mut v3rho2tau),
            np_(has_kxc, &mut v3rhosigma2), np_(has_kxc, &mut v3rhosigmalapl),
            np_(has_kxc, &mut v3rhosigmatau),
            np_(has_kxc, &mut v3rholapl2), np_(has_kxc, &mut v3rholapltau),
            np_(has_kxc, &mut v3rhotau2),
            np_(has_kxc, &mut v3sigma3), np_(has_kxc, &mut v3sigma2lapl),
            np_(has_kxc, &mut v3sigma2tau),
            np_(has_kxc, &mut v3sigmalapl2), np_(has_kxc, &mut v3sigmalapltau),
            np_(has_kxc, &mut v3sigmatau2),
            np_(has_kxc, &mut v3lapl3), np_(has_kxc, &mut v3lapl2tau),
            np_(has_kxc, &mut v3lapltau2), np_(has_kxc, &mut v3tau3),
        );

        if flags & FLAGS_HAVE_LXC != 0 {
            oracle_ffi::xc_mgga_lxc(
                func, np,
                rho.as_ptr(), sigma.as_ptr(), lapl.as_ptr(), tau.as_ptr(),
                v4rho4.as_mut_ptr(), v4rho3sigma.as_mut_ptr(), v4rho3lapl.as_mut_ptr(), v4rho3tau.as_mut_ptr(),
                v4rho2sigma2.as_mut_ptr(), v4rho2sigmalapl.as_mut_ptr(), v4rho2sigmatau.as_mut_ptr(),
                v4rho2lapl2.as_mut_ptr(), v4rho2lapltau.as_mut_ptr(), v4rho2tau2.as_mut_ptr(),
                v4rhosigma3.as_mut_ptr(), v4rhosigma2lapl.as_mut_ptr(), v4rhosigma2tau.as_mut_ptr(),
                v4rhosigmalapl2.as_mut_ptr(), v4rhosigmalapltau.as_mut_ptr(), v4rhosigmatau2.as_mut_ptr(),
                v4rholapl3.as_mut_ptr(), v4rholapl2tau.as_mut_ptr(), v4rholapltau2.as_mut_ptr(), v4rhotau3.as_mut_ptr(),
                v4sigma4.as_mut_ptr(), v4sigma3lapl.as_mut_ptr(), v4sigma3tau.as_mut_ptr(),
                v4sigma2lapl2.as_mut_ptr(), v4sigma2lapltau.as_mut_ptr(), v4sigma2tau2.as_mut_ptr(),
                v4sigmalapl3.as_mut_ptr(), v4sigmalapl2tau.as_mut_ptr(), v4sigmalapltau2.as_mut_ptr(), v4sigmatau3.as_mut_ptr(),
                v4lapl4.as_mut_ptr(), v4lapl3tau.as_mut_ptr(), v4lapl2tau2.as_mut_ptr(), v4lapltau3.as_mut_ptr(), v4tau4.as_mut_ptr(),
            );
        }

        oracle_ffi::xc_func_end(func);
        oracle_ffi::xc_func_free(func);
    }

    Ok(MggaOracleOutput {
        zk, vrho, vsigma, vlapl, vtau,
        v2rho2, v2rhosigma, v2rholapl, v2rhotau,
        v2sigma2, v2sigmalapl, v2sigmatau,
        v2lapl2, v2lapltau, v2tau2,
        v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau,
        v3rhosigma2, v3rhosigmalapl, v3rhosigmatau,
        v3rholapl2, v3rholapltau, v3rhotau2,
        v3sigma3, v3sigma2lapl, v3sigma2tau,
        v3sigmalapl2, v3sigmalapltau, v3sigmatau2,
        v3lapl3, v3lapl2tau, v3lapltau2, v3tau3,
        v4rho4, v4rho3sigma, v4rho3lapl, v4rho3tau,
        v4rho2sigma2, v4rho2sigmalapl, v4rho2sigmatau,
        v4rho2lapl2, v4rho2lapltau, v4rho2tau2,
        v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau,
        v4rhosigmalapl2, v4rhosigmalapltau, v4rhosigmatau2,
        v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3,
        v4sigma4, v4sigma3lapl, v4sigma3tau,
        v4sigma2lapl2, v4sigma2lapltau, v4sigma2tau2,
        v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3,
        v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4,
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
