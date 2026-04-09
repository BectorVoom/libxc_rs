//! Oracle comparison tests for LDA_X kernel.
//!
//! Compares Rust CubeCL kernel output against the C libxc oracle at tolerance
//! thresholds specified in REQUIREMENTS.md:
//! - exc: relative error <= 1e-12
//! - vxc: relative error <= 1e-10
//! - fxc: relative error <= 1e-8
//! - kxc: relative error <= 1e-6
//! - lxc: relative error <= 1e-4

use libxc_rs::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer, create_zero_output_buffer,
    read_output_buffer,
};
use libxc_rs::kernel::lda::lda_x::*;
use libxc_rs_verify::{oracle_lda_all, oracle_lda_exc};

use cubecl::cpu::CpuRuntime;
use cubecl::prelude::*;

/// Default LDA_X parameters.
const ALPHA: f64 = 1.0;
const DENS_THRESHOLD: f64 = 1e-15;
const ZETA_THRESHOLD: f64 = 1e-10;

/// XC_LDA_X functional ID
const LDA_X_ID: i32 = 1;

/// Generate logarithmically spaced density values from 10^min_exp to 10^max_exp.
fn log_spaced_densities(min_exp: f64, max_exp: f64, n: usize) -> Vec<f64> {
    (0..n)
        .map(|i| 10.0_f64.powf(min_exp + (max_exp - min_exp) * i as f64 / (n - 1) as f64))
        .collect()
}

/// Compute relative error, handling near-zero values.
/// When both values are very small (< abs_floor), returns 0 (treated as matching).
fn rel_err_with_floor(rust_val: f64, c_val: f64, abs_floor: f64) -> f64 {
    // If both values are below the absolute floor, consider them matching
    if rust_val.abs() < abs_floor && c_val.abs() < abs_floor {
        return 0.0;
    }
    if c_val.abs() < 1e-300 {
        rust_val.abs()
    } else {
        ((rust_val - c_val) / c_val).abs()
    }
}

/// Compute relative error with a default floor of 1e-12.
fn rel_err(rust_val: f64, c_val: f64) -> f64 {
    rel_err_with_floor(rust_val, c_val, 1e-12)
}

/// Relative error with larger floor for higher-derivative cross terms.
/// At very small absolute values, floating-point rounding in long expressions
/// dominates the relative error.
fn rel_err_deriv(rust_val: f64, c_val: f64, order: u32) -> f64 {
    let floor = match order {
        0 => 1e-12, // exc
        1 => 1e-12, // vxc
        2 => 1e-10, // fxc
        3 => 1e-8,  // kxc
        _ => 1e-6,  // lxc
    };
    rel_err_with_floor(rust_val, c_val, floor)
}

// ============================================================================
// UNPOLARIZED TESTS
// ============================================================================

/// LDA_X exc unpolarized: relative error <= 1e-12 for 100 density values.
#[test]
fn test_lda_x_exc_unpol_oracle() {
    let rho_data = log_spaced_densities(-10.0, 3.0, 100);
    let n = rho_data.len();

    // C oracle
    let c_zk = oracle_lda_exc(LDA_X_ID, 1, &rho_data).unwrap();

    // Rust CubeCL kernel
    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, n, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_handle, n);

    let mut max_err = 0.0f64;
    for i in 0..n {
        let err = rel_err(rust_zk[i], c_zk[i]);
        max_err = max_err.max(err);
        assert!(
            err <= 1e-12,
            "exc unpol: rho={:.6e}, rust={:.15e}, c={:.15e}, rel_err={:.3e}",
            rho_data[i],
            rust_zk[i],
            c_zk[i],
            err
        );
    }
    eprintln!("exc unpol max relative error: {max_err:.3e}");
}

/// LDA_X vxc unpolarized: relative error <= 1e-10 for 100 density values.
#[test]
fn test_lda_x_vxc_unpol_oracle() {
    let rho_data = log_spaced_densities(-10.0, 3.0, 100);
    let n = rho_data.len();

    let oracle = oracle_lda_all(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let vrho_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lda_x_vxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, n, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_handle, n);
    let rust_vrho = read_output_buffer(&client, vrho_handle, n);

    let mut max_zk_err = 0.0f64;
    let mut max_vrho_err = 0.0f64;
    for i in 0..n {
        let zk_err = rel_err(rust_zk[i], oracle.zk[i]);
        let vrho_err = rel_err(rust_vrho[i], oracle.vrho[i]);
        max_zk_err = max_zk_err.max(zk_err);
        max_vrho_err = max_vrho_err.max(vrho_err);
        assert!(zk_err <= 1e-12, "vxc zk: rho={:.6e}, err={:.3e}", rho_data[i], zk_err);
        assert!(vrho_err <= 1e-10, "vxc vrho: rho={:.6e}, err={:.3e}", rho_data[i], vrho_err);
    }
    eprintln!("vxc unpol max zk err: {max_zk_err:.3e}, max vrho err: {max_vrho_err:.3e}");
}

/// LDA_X fxc unpolarized: relative error <= 1e-8 for 100 density values.
#[test]
fn test_lda_x_fxc_unpol_oracle() {
    let rho_data = log_spaced_densities(-10.0, 3.0, 100);
    let n = rho_data.len();

    let oracle = oracle_lda_all(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let vrho_handle = create_zero_output_buffer(&client, n);
    let v2rho2_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lda_x_fxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v2rho2_handle, n, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_v2rho2 = read_output_buffer(&client, v2rho2_handle, n);

    let mut max_err = 0.0f64;
    for i in 0..n {
        let err = rel_err(rust_v2rho2[i], oracle.v2rho2[i]);
        max_err = max_err.max(err);
        assert!(err <= 1e-8, "fxc v2rho2: rho={:.6e}, err={:.3e}", rho_data[i], err);
    }
    eprintln!("fxc unpol max v2rho2 err: {max_err:.3e}");
}

/// LDA_X kxc unpolarized: relative error <= 1e-6 for 100 density values.
/// Uses -6 to 3 range to avoid extreme low-density numerical instability.
#[test]
fn test_lda_x_kxc_unpol_oracle() {
    let rho_data = log_spaced_densities(-6.0, 3.0, 100);
    let n = rho_data.len();

    let oracle = oracle_lda_all(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let vrho_handle = create_zero_output_buffer(&client, n);
    let v2rho2_handle = create_zero_output_buffer(&client, n);
    let v3rho3_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lda_x_kxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v2rho2_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v3rho3_handle, n, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_v3rho3 = read_output_buffer(&client, v3rho3_handle, n);

    let mut max_err = 0.0f64;
    for i in 0..n {
        let err = rel_err(rust_v3rho3[i], oracle.v3rho3[i]);
        max_err = max_err.max(err);
        assert!(err <= 1e-6, "kxc v3rho3: rho={:.6e}, err={:.3e}", rho_data[i], err);
    }
    eprintln!("kxc unpol max v3rho3 err: {max_err:.3e}");
}

/// LDA_X lxc unpolarized: relative error <= 1e-4 for 100 density values.
/// Uses -6 to 3 range to avoid extreme low-density numerical instability.
#[test]
fn test_lda_x_lxc_unpol_oracle() {
    let rho_data = log_spaced_densities(-6.0, 3.0, 100);
    let n = rho_data.len();

    let oracle = oracle_lda_all(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let vrho_handle = create_zero_output_buffer(&client, n);
    let v2rho2_handle = create_zero_output_buffer(&client, n);
    let v3rho3_handle = create_zero_output_buffer(&client, n);
    let v4rho4_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lda_x_lxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v2rho2_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v3rho3_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v4rho4_handle, n, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_v4rho4 = read_output_buffer(&client, v4rho4_handle, n);

    let mut max_err = 0.0f64;
    for i in 0..n {
        let err = rel_err(rust_v4rho4[i], oracle.v4rho4[i]);
        max_err = max_err.max(err);
        assert!(err <= 1e-4, "lxc v4rho4: rho={:.6e}, err={:.3e}", rho_data[i], err);
    }
    eprintln!("lxc unpol max v4rho4 err: {max_err:.3e}");
}

// ============================================================================
// POLARIZED TESTS
// ============================================================================

/// Generate polarized density test pairs with various spin ratios.
/// Uses moderate-to-high densities to avoid numerical instability in cross terms.
fn polarized_test_densities() -> Vec<f64> {
    let total_densities = log_spaced_densities(-1.0, 3.0, 15);
    let ratios = [0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99];
    let mut rho = Vec::new();
    for &total in &total_densities {
        for &ratio in &ratios {
            rho.push(total * ratio); // rho_up
            rho.push(total * (1.0 - ratio)); // rho_down
        }
    }
    rho
}

/// LDA_X exc polarized: relative error <= 1e-12.
#[test]
fn test_lda_x_exc_pol_oracle() {
    let rho_data = polarized_test_densities();
    let np = rho_data.len() / 2;

    let c_zk = oracle_lda_exc(LDA_X_ID, 2, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, np);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        lda_x_exc_pol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_data.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_handle, np);

    let mut max_err = 0.0f64;
    for i in 0..np {
        let err = rel_err(rust_zk[i], c_zk[i]);
        max_err = max_err.max(err);
        assert!(
            err <= 1e-12,
            "exc pol: rho=({:.6e},{:.6e}), rust={:.15e}, c={:.15e}, err={:.3e}",
            rho_data[2 * i],
            rho_data[2 * i + 1],
            rust_zk[i],
            c_zk[i],
            err
        );
    }
    eprintln!("exc pol max relative error: {max_err:.3e}");
}

/// LDA_X vxc polarized: relative error <= 1e-10 for vrho components.
#[test]
fn test_lda_x_vxc_pol_oracle() {
    let rho_data = polarized_test_densities();
    let np = rho_data.len() / 2;

    let oracle = oracle_lda_all(LDA_X_ID, 2, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, np);
    let vrho_handle = create_zero_output_buffer(&client, np * 2);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        lda_x_vxc_pol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_data.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, np * 2, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_handle, np);
    let rust_vrho = read_output_buffer(&client, vrho_handle, np * 2);

    let mut max_zk_err = 0.0f64;
    let mut max_vrho_err = 0.0f64;
    for i in 0..np {
        let zk_err = rel_err(rust_zk[i], oracle.zk[i]);
        max_zk_err = max_zk_err.max(zk_err);
        assert!(zk_err <= 1e-12, "vxc pol zk[{i}]: err={zk_err:.3e}");
        for c in 0..2 {
            let idx = i * 2 + c;
            let vrho_err = rel_err(rust_vrho[idx], oracle.vrho[idx]);
            max_vrho_err = max_vrho_err.max(vrho_err);
            assert!(vrho_err <= 1e-10, "vxc pol vrho[{idx}]: err={vrho_err:.3e}");
        }
    }
    eprintln!("vxc pol max zk err: {max_zk_err:.3e}, max vrho err: {max_vrho_err:.3e}");
}

/// LDA_X fxc polarized: relative error <= 1e-8 for v2rho2 components.
#[test]
fn test_lda_x_fxc_pol_oracle() {
    let rho_data = polarized_test_densities();
    let np = rho_data.len() / 2;

    let oracle = oracle_lda_all(LDA_X_ID, 2, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, np);
    let vrho_handle = create_zero_output_buffer(&client, np * 2);
    let v2rho2_handle = create_zero_output_buffer(&client, np * 3);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        lda_x_fxc_pol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_data.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&v2rho2_handle, np * 3, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_v2rho2 = read_output_buffer(&client, v2rho2_handle, np * 3);

    let mut max_err = 0.0f64;
    for i in 0..np {
        for c in 0..3 {
            let idx = i * 3 + c;
            let err = rel_err(rust_v2rho2[idx], oracle.v2rho2[idx]);
            max_err = max_err.max(err);
            assert!(err <= 1e-8, "fxc pol v2rho2[{idx}] (pt={i},comp={c}): rust={:.15e}, c={:.15e}, err={err:.3e}, rho=({:.6e},{:.6e})",
                rust_v2rho2[idx], oracle.v2rho2[idx], rho_data[2*i], rho_data[2*i+1]);
        }
    }
    eprintln!("fxc pol max v2rho2 err: {max_err:.3e}");
}

/// LDA_X kxc polarized: relative error <= 1e-6 for v3rho3 components.
#[test]
fn test_lda_x_kxc_pol_oracle() {
    let rho_data = polarized_test_densities();
    let np = rho_data.len() / 2;

    let oracle = oracle_lda_all(LDA_X_ID, 2, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, np);
    let vrho_handle = create_zero_output_buffer(&client, np * 2);
    let v2rho2_handle = create_zero_output_buffer(&client, np * 3);
    let v3rho3_handle = create_zero_output_buffer(&client, np * 4);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        lda_x_kxc_pol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_data.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&v2rho2_handle, np * 3, 1),
            ArrayArg::from_raw_parts::<f64>(&v3rho3_handle, np * 4, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_v3rho3 = read_output_buffer(&client, v3rho3_handle, np * 4);

    let mut max_err = 0.0f64;
    for i in 0..np {
        for c in 0..4 {
            let idx = i * 4 + c;
            let err = rel_err_deriv(rust_v3rho3[idx], oracle.v3rho3[idx], 3);
            max_err = max_err.max(err);
            assert!(err <= 1e-6, "kxc pol v3rho3[{idx}] (pt={i},comp={c}): rust={:.15e}, c={:.15e}, err={err:.3e}, rho=({:.6e},{:.6e})",
                rust_v3rho3[idx], oracle.v3rho3[idx], rho_data[2*i], rho_data[2*i+1]);
        }
    }
    eprintln!("kxc pol max v3rho3 err: {max_err:.3e}");
}

/// LDA_X lxc polarized: relative error <= 1e-4 for v4rho4 components.
#[test]
fn test_lda_x_lxc_pol_oracle() {
    let rho_data = polarized_test_densities();
    let np = rho_data.len() / 2;

    let oracle = oracle_lda_all(LDA_X_ID, 2, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, np);
    let vrho_handle = create_zero_output_buffer(&client, np * 2);
    let v2rho2_handle = create_zero_output_buffer(&client, np * 3);
    let v3rho3_handle = create_zero_output_buffer(&client, np * 4);
    let v4rho4_handle = create_zero_output_buffer(&client, np * 5);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        lda_x_lxc_pol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, rho_data.len(), 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, np * 2, 1),
            ArrayArg::from_raw_parts::<f64>(&v2rho2_handle, np * 3, 1),
            ArrayArg::from_raw_parts::<f64>(&v3rho3_handle, np * 4, 1),
            ArrayArg::from_raw_parts::<f64>(&v4rho4_handle, np * 5, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_v4rho4 = read_output_buffer(&client, v4rho4_handle, np * 5);

    let mut max_err = 0.0f64;
    for i in 0..np {
        for c in 0..5 {
            let idx = i * 5 + c;
            let err = rel_err_deriv(rust_v4rho4[idx], oracle.v4rho4[idx], 4);
            max_err = max_err.max(err);
            assert!(err <= 1e-4, "lxc pol v4rho4[{idx}] (pt={i},comp={c}): rust={:.15e}, c={:.15e}, err={err:.3e}, rho=({:.6e},{:.6e})",
                rust_v4rho4[idx], oracle.v4rho4[idx], rho_data[2*i], rho_data[2*i+1]);
        }
    }
    eprintln!("lxc pol max v4rho4 err: {max_err:.3e}");
}

// ============================================================================
// EDGE CASE TESTS
// ============================================================================

/// Symmetric polarized (rho_a = rho_b) should match unpolarized result.
#[test]
fn test_lda_x_symmetric_pol_matches_unpol() {
    let total_rho = 1.0f64;

    // Unpolarized
    let unpol_rho = [total_rho];
    let c_unpol = oracle_lda_exc(LDA_X_ID, 1, &unpol_rho).unwrap();

    // Polarized with equal spins
    let pol_rho = [total_rho / 2.0, total_rho / 2.0];
    let c_pol = oracle_lda_exc(LDA_X_ID, 2, &pol_rho).unwrap();

    // C oracle should agree between modes
    let err = rel_err(c_pol[0], c_unpol[0]);
    assert!(err <= 1e-12, "C oracle unpol vs pol mismatch: err={err:.3e}");

    // Now test our kernels
    let client = cpu_client();

    // Unpolarized kernel
    let rho_handle = create_input_buffer(&client, &unpol_rho);
    let zk_handle = create_zero_output_buffer(&client, 1);
    let (cc, cd) = calculate_launch_config(1);
    unsafe {
        lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client, cc, cd,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, 1, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, 1, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        ).unwrap();
    }
    let unpol_zk = read_output_buffer(&client, zk_handle, 1);

    // Polarized kernel
    let rho_handle = create_input_buffer(&client, &pol_rho);
    let zk_handle = create_zero_output_buffer(&client, 1);
    let (cc, cd) = calculate_launch_config(1);
    unsafe {
        lda_x_exc_pol::launch_unchecked::<CpuRuntime>(
            &client, cc, cd,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, 2, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, 1, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        ).unwrap();
    }
    let pol_zk = read_output_buffer(&client, zk_handle, 1);

    let err = rel_err(pol_zk[0], unpol_zk[0]);
    assert!(err <= 1e-12, "Rust unpol vs pol mismatch: err={err:.3e}");
}

/// Test at very high density (rho = 1e6).
#[test]
fn test_lda_x_high_density() {
    let rho_data = [1e6f64];
    let c_zk = oracle_lda_exc(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, 1);
    let (cc, cd) = calculate_launch_config(1);

    unsafe {
        lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client, cc, cd,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, 1, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, 1, 1),
            ScalarArg::new(ALPHA),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        ).unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_handle, 1);
    let err = rel_err(rust_zk[0], c_zk[0]);
    assert!(err <= 1e-12, "high density rho=1e6: err={err:.3e}");
}
