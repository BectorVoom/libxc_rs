//! Stress tests for LDA_X canary kernel.
//!
//! Pushes the LDA_X kernel to boundary conditions NOT covered by the main
//! oracle comparison in `lda_x_oracle.rs`:
//! - Threshold boundary behavior (densities at/near dens_threshold)
//! - Non-default alpha (ext_params scaling)
//! - Extreme density regimes (22 orders of magnitude)
//! - Highly asymmetric spin polarizations (zeta near +/-1)
//! - Symmetric polarized == unpolarized self-consistency
//! - Large batch dispatch (10000 grid points)

use libxc_rs::kernel::launch::{
    calculate_launch_config, cpu_client, create_input_buffer, create_zero_output_buffer,
    read_output_buffer,
};
use libxc_rs::kernel::lda::lda_x::*;
use libxc_rs_verify::oracle_lda_exc;

use cubecl::cpu::CpuRuntime;
use cubecl::prelude::*;

/// XC_LDA_X functional ID.
const LDA_X_ID: i32 = 1;

/// Default parameters.
const ALPHA: f64 = 1.0;
const DENS_THRESHOLD: f64 = 1e-15;
const ZETA_THRESHOLD: f64 = 1e-10;

/// Compute relative error, handling near-zero values.
fn rel_err_with_floor(rust_val: f64, c_val: f64, abs_floor: f64) -> f64 {
    if rust_val.abs() < abs_floor && c_val.abs() < abs_floor {
        return 0.0;
    }
    if c_val.abs() < 1e-300 {
        rust_val.abs()
    } else {
        ((rust_val - c_val) / c_val).abs()
    }
}

fn rel_err(rust_val: f64, c_val: f64) -> f64 {
    rel_err_with_floor(rust_val, c_val, 1e-12)
}

fn rel_err_deriv(rust_val: f64, c_val: f64, order: u32) -> f64 {
    let floor = match order {
        0 => 1e-12,
        1 => 1e-12,
        2 => 1e-10,
        3 => 1e-8,
        _ => 1e-6,
    };
    rel_err_with_floor(rust_val, c_val, floor)
}

// ============================================================================
// 1. THRESHOLD BOUNDARY TESTS
// ============================================================================

/// Test densities around dens_threshold (1e-15).
/// Below threshold: Rust kernel should produce zero zk.
/// Above threshold: must match oracle.
#[test]
fn test_lda_x_threshold_boundary() {
    // Densities around 1e-15 boundary
    let densities = [0.5e-15, 0.9e-15, 1.0e-15, 1.1e-15, 2.0e-15, 1e-14];
    let n = densities.len();

    // Oracle uses same functional but note: C libxc has its own default
    // dens_threshold which may differ. We compare Rust output with the Rust
    // threshold applied to C oracle output.
    let c_zk = oracle_lda_exc(LDA_X_ID, 1, &densities).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &densities);
    let zk_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        exc_unpol::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
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

    for i in 0..n {
        if densities[i] < DENS_THRESHOLD {
            // Below threshold: CubeCL uses branchless select, so it may not
            // produce exact zero. But the output should still match the C oracle
            // (which also has a threshold). We just check it is very small or
            // matches C.
            let err = rel_err_with_floor(rust_zk[i], c_zk[i], 1e-30);
            assert!(
                err <= 1e-10 || rust_zk[i].abs() < 1e-20,
                "threshold below: rho={:.2e}, rust_zk={:.6e}, c_zk={:.6e}, err={:.3e}",
                densities[i], rust_zk[i], c_zk[i], err
            );
        } else {
            // Above threshold: must match oracle
            let err = rel_err(rust_zk[i], c_zk[i]);
            assert!(
                err <= 1e-12,
                "threshold above: rho={:.2e}, rust_zk={:.6e}, c_zk={:.6e}, err={:.3e}",
                densities[i], rust_zk[i], c_zk[i], err
            );
        }
    }
    eprintln!("threshold boundary test passed");
}

/// Test all derivative orders at densities just above threshold.
#[test]
fn test_lda_x_derivatives_at_threshold() {
    // Densities just above 1e-15 and well above
    let rho_data = [2e-15, 1e-14, 1e-13, 1e-12];
    let n = rho_data.len();

    let oracle = libxc_rs_verify::oracle_lda_all(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let vrho_handle = create_zero_output_buffer(&client, n);
    let v2rho2_handle = create_zero_output_buffer(&client, n);
    let v3rho3_handle = create_zero_output_buffer(&client, n);
    let v4rho4_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lxc_unpol::lda_x_lxc_unpol::launch_unchecked::<CpuRuntime>(
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

    let rust_zk = read_output_buffer(&client, zk_handle, n);
    let rust_vrho = read_output_buffer(&client, vrho_handle, n);
    let rust_v2rho2 = read_output_buffer(&client, v2rho2_handle, n);
    let rust_v3rho3 = read_output_buffer(&client, v3rho3_handle, n);
    let rust_v4rho4 = read_output_buffer(&client, v4rho4_handle, n);

    for i in 0..n {
        let zk_err = rel_err(rust_zk[i], oracle.zk[i]);
        assert!(zk_err <= 1e-12, "near-threshold zk: rho={:.2e}, err={:.3e}", rho_data[i], zk_err);

        let vrho_err = rel_err(rust_vrho[i], oracle.vrho[i]);
        assert!(vrho_err <= 1e-10, "near-threshold vrho: rho={:.2e}, err={:.3e}", rho_data[i], vrho_err);

        let v2_err = rel_err(rust_v2rho2[i], oracle.v2rho2[i]);
        assert!(v2_err <= 1e-8, "near-threshold v2rho2: rho={:.2e}, err={:.3e}", rho_data[i], v2_err);

        let v3_err = rel_err_deriv(rust_v3rho3[i], oracle.v3rho3[i], 3);
        assert!(v3_err <= 1e-6, "near-threshold v3rho3: rho={:.2e}, err={:.3e}", rho_data[i], v3_err);

        let v4_err = rel_err_deriv(rust_v4rho4[i], oracle.v4rho4[i], 4);
        assert!(v4_err <= 1e-4, "near-threshold v4rho4: rho={:.2e}, err={:.3e}", rho_data[i], v4_err);
    }
    eprintln!("derivatives at threshold boundary test passed");
}

// ============================================================================
// 2. NON-DEFAULT ALPHA (EXT_PARAMS) TESTS
// ============================================================================

/// Test LDA_X with non-default alpha scaling parameter.
/// LDA_X energy is linear in alpha: zk(alpha) = alpha * zk(1).
/// Note: C libxc LDA_X (ID=1) does NOT expose alpha as an ext_param
/// (it's hardcoded to 1.0), so we verify the Rust kernel's alpha handling
/// via self-consistency (linearity) rather than oracle comparison.
#[test]
fn test_lda_x_nondefault_alpha() {
    let alpha_values = [0.0, 0.25, 0.5, 1.0, 1.5, 2.0];
    let rho_data = [0.01, 0.1, 1.0, 10.0, 100.0];
    let n = rho_data.len();

    // Get reference at alpha=1.0 (which matches oracle)
    let c_zk_ref = oracle_lda_exc(LDA_X_ID, 1, &rho_data).unwrap();

    for &alpha in &alpha_values {
        let client = cpu_client();
        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, n);
        let (cube_count, cube_dim) = calculate_launch_config(n);

        unsafe {
            exc_unpol::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
                &client,
                cube_count,
                cube_dim,
                ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
                ArrayArg::from_raw_parts::<f64>(&zk_handle, n, 1),
                ScalarArg::new(alpha),
                ScalarArg::new(DENS_THRESHOLD),
                ScalarArg::new(ZETA_THRESHOLD),
            )
            .unwrap();
        }

        let rust_zk = read_output_buffer(&client, zk_handle, n);

        for i in 0..n {
            if alpha == 0.0 {
                // alpha=0 should give zero exchange energy
                assert!(
                    rust_zk[i].abs() < 1e-30,
                    "alpha=0: rho={:.2e}, rust_zk={:.6e} (expected ~0)",
                    rho_data[i], rust_zk[i]
                );
            } else {
                // Verify linearity: zk(alpha) = alpha * zk(1)
                let expected = alpha * c_zk_ref[i];
                let err = rel_err(rust_zk[i], expected);
                assert!(
                    err <= 1e-12,
                    "alpha={alpha}: rho={:.2e}, rust={:.15e}, expected={:.15e}, err={:.3e}",
                    rho_data[i], rust_zk[i], expected, err
                );
            }
        }
    }
    eprintln!("non-default alpha test passed for all alpha values");
}

/// Test alpha scaling: alpha=2.0 should produce 2x the energy of alpha=1.0.
#[test]
fn test_lda_x_alpha_linearity() {
    let rho_data = [0.1, 1.0, 10.0];
    let n = rho_data.len();

    let client = cpu_client();

    // alpha=1.0
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk1_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);
    unsafe {
        exc_unpol::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk1_handle, n, 1),
            ScalarArg::new(1.0),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        ).unwrap();
    }
    let zk_alpha1 = read_output_buffer(&client, zk1_handle, n);

    // alpha=2.0
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk2_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);
    unsafe {
        exc_unpol::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
            &client, cube_count, cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk2_handle, n, 1),
            ScalarArg::new(2.0),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        ).unwrap();
    }
    let zk_alpha2 = read_output_buffer(&client, zk2_handle, n);

    for i in 0..n {
        let expected = 2.0 * zk_alpha1[i];
        let err = rel_err(zk_alpha2[i], expected);
        assert!(
            err <= 1e-14,
            "alpha linearity: rho={:.2e}, 2*zk1={:.15e}, zk2={:.15e}, err={:.3e}",
            rho_data[i], expected, zk_alpha2[i], err
        );
    }
    eprintln!("alpha linearity test passed");
}

/// Test non-default alpha with all derivative orders.
/// Since LDA_X energy/derivatives are linear in alpha, we verify:
///   deriv(alpha=0.5) = 0.5 * deriv(alpha=1.0)
/// The alpha=1.0 results are validated against the C oracle in lda_x_oracle.rs.
#[test]
fn test_lda_x_alpha_derivatives() {
    let alpha = 0.5;
    let rho_data = [0.1, 1.0, 10.0, 100.0];
    let n = rho_data.len();

    // Reference: alpha=1.0 (validated against C oracle)
    let oracle_ref = libxc_rs_verify::oracle_lda_all(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();

    // Run with alpha=0.5
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let vrho_handle = create_zero_output_buffer(&client, n);
    let v2rho2_handle = create_zero_output_buffer(&client, n);
    let v3rho3_handle = create_zero_output_buffer(&client, n);
    let v4rho4_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lxc_unpol::lda_x_lxc_unpol::launch_unchecked::<CpuRuntime>(
            &client,
            cube_count,
            cube_dim,
            ArrayArg::from_raw_parts::<f64>(&rho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&zk_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&vrho_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v2rho2_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v3rho3_handle, n, 1),
            ArrayArg::from_raw_parts::<f64>(&v4rho4_handle, n, 1),
            ScalarArg::new(alpha),
            ScalarArg::new(DENS_THRESHOLD),
            ScalarArg::new(ZETA_THRESHOLD),
        )
        .unwrap();
    }

    let rust_zk = read_output_buffer(&client, zk_handle, n);
    let rust_vrho = read_output_buffer(&client, vrho_handle, n);
    let rust_v2rho2 = read_output_buffer(&client, v2rho2_handle, n);
    let rust_v3rho3 = read_output_buffer(&client, v3rho3_handle, n);
    let rust_v4rho4 = read_output_buffer(&client, v4rho4_handle, n);

    for i in 0..n {
        // All derivatives should be alpha * reference (linearity)
        let zk_err = rel_err(rust_zk[i], alpha * oracle_ref.zk[i]);
        assert!(zk_err <= 1e-12, "alpha={alpha} zk: rho={:.2e}, err={:.3e}", rho_data[i], zk_err);

        let vrho_err = rel_err(rust_vrho[i], alpha * oracle_ref.vrho[i]);
        assert!(vrho_err <= 1e-10, "alpha={alpha} vrho: rho={:.2e}, err={:.3e}", rho_data[i], vrho_err);

        let v2_err = rel_err(rust_v2rho2[i], alpha * oracle_ref.v2rho2[i]);
        assert!(v2_err <= 1e-8, "alpha={alpha} v2rho2: rho={:.2e}, err={:.3e}", rho_data[i], v2_err);

        let v3_expected = alpha * oracle_ref.v3rho3[i];
        let v3_err = rel_err_deriv(rust_v3rho3[i], v3_expected, 3);
        assert!(v3_err <= 1e-6, "alpha={alpha} v3rho3: rho={:.2e}, err={:.3e}", rho_data[i], v3_err);

        let v4_expected = alpha * oracle_ref.v4rho4[i];
        let v4_err = rel_err_deriv(rust_v4rho4[i], v4_expected, 4);
        assert!(v4_err <= 1e-4, "alpha={alpha} v4rho4: rho={:.2e}, err={:.3e}", rho_data[i], v4_err);
    }
    eprintln!("alpha=0.5 all derivatives test passed (linearity check)");
}

// ============================================================================
// 3. EXTREME DENSITY TESTS
// ============================================================================

/// Test LDA_X across 22 orders of magnitude in density.
/// Verifies no NaN, no Inf, and oracle match for all derivatives.
#[test]
fn test_lda_x_extreme_density() {
    let rho_data: Vec<f64> = vec![
        1e-14, 1e-12, 1e-10, 1e-8, 1e-6, 1e-4, 1e-2, 1.0, 1e2, 1e4, 1e6, 1e8,
    ];
    let n = rho_data.len();

    let oracle = libxc_rs_verify::oracle_lda_all(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let vrho_handle = create_zero_output_buffer(&client, n);
    let v2rho2_handle = create_zero_output_buffer(&client, n);
    let v3rho3_handle = create_zero_output_buffer(&client, n);
    let v4rho4_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    unsafe {
        lxc_unpol::lda_x_lxc_unpol::launch_unchecked::<CpuRuntime>(
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

    let rust_zk = read_output_buffer(&client, zk_handle, n);
    let rust_vrho = read_output_buffer(&client, vrho_handle, n);
    let rust_v2rho2 = read_output_buffer(&client, v2rho2_handle, n);
    let rust_v3rho3 = read_output_buffer(&client, v3rho3_handle, n);
    let rust_v4rho4 = read_output_buffer(&client, v4rho4_handle, n);

    let mut max_zk_err = 0.0f64;
    for i in 0..n {
        // No NaN or Inf
        assert!(!rust_zk[i].is_nan(), "NaN in zk at rho={:.2e}", rho_data[i]);
        assert!(!rust_zk[i].is_infinite(), "Inf in zk at rho={:.2e}", rho_data[i]);
        assert!(!rust_vrho[i].is_nan(), "NaN in vrho at rho={:.2e}", rho_data[i]);
        assert!(!rust_vrho[i].is_infinite(), "Inf in vrho at rho={:.2e}", rho_data[i]);
        assert!(!rust_v2rho2[i].is_nan(), "NaN in v2rho2 at rho={:.2e}", rho_data[i]);
        assert!(!rust_v2rho2[i].is_infinite(), "Inf in v2rho2 at rho={:.2e}", rho_data[i]);
        assert!(!rust_v3rho3[i].is_nan(), "NaN in v3rho3 at rho={:.2e}", rho_data[i]);
        assert!(!rust_v3rho3[i].is_infinite(), "Inf in v3rho3 at rho={:.2e}", rho_data[i]);
        assert!(!rust_v4rho4[i].is_nan(), "NaN in v4rho4 at rho={:.2e}", rho_data[i]);
        assert!(!rust_v4rho4[i].is_infinite(), "Inf in v4rho4 at rho={:.2e}", rho_data[i]);

        // Oracle comparison for exc
        let zk_err = rel_err(rust_zk[i], oracle.zk[i]);
        max_zk_err = max_zk_err.max(zk_err);
        assert!(
            zk_err <= 1e-12,
            "extreme density zk: rho={:.2e}, rust={:.15e}, c={:.15e}, err={:.3e}",
            rho_data[i], rust_zk[i], oracle.zk[i], zk_err
        );

        // Oracle comparison for vrho
        let vrho_err = rel_err(rust_vrho[i], oracle.vrho[i]);
        assert!(vrho_err <= 1e-10, "extreme density vrho: rho={:.2e}, err={:.3e}", rho_data[i], vrho_err);

        // Higher derivatives with appropriate floors
        let v2_err = rel_err(rust_v2rho2[i], oracle.v2rho2[i]);
        assert!(v2_err <= 1e-8, "extreme density v2rho2: rho={:.2e}, err={:.3e}", rho_data[i], v2_err);

        let v3_err = rel_err_deriv(rust_v3rho3[i], oracle.v3rho3[i], 3);
        assert!(v3_err <= 1e-6, "extreme density v3rho3: rho={:.2e}, err={:.3e}", rho_data[i], v3_err);

        let v4_err = rel_err_deriv(rust_v4rho4[i], oracle.v4rho4[i], 4);
        assert!(v4_err <= 1e-4, "extreme density v4rho4: rho={:.2e}, err={:.3e}", rho_data[i], v4_err);
    }
    eprintln!("extreme density test passed, max zk err: {max_zk_err:.3e}");
}

// ============================================================================
// 4. ASYMMETRIC SPIN POLARIZATION TESTS
// ============================================================================

/// Test highly asymmetric spin polarizations.
/// These stress the zeta computation and spin-dependent branches.
#[test]
fn test_lda_x_pol_asymmetric() {
    // (rho_a, rho_b) pairs with extreme asymmetry
    let spin_pairs: Vec<(f64, f64)> = vec![
        (0.999, 0.001),
        (0.001, 0.999),
        (0.9, 0.1),
        (0.1, 0.9),
        (1e-8, 1.0),
        (1.0, 1e-8),
        (1e-4, 1e-1),
        (1e-1, 1e-4),
        (0.9999, 0.0001),
        (10.0, 1e-6),
    ];

    for &(rho_a, rho_b) in &spin_pairs {
        let rho_data = [rho_a, rho_b];
        let np = 1;

        let c_zk = oracle_lda_exc(LDA_X_ID, 2, &rho_data).unwrap();

        let client = cpu_client();
        let rho_handle = create_input_buffer(&client, &rho_data);
        let zk_handle = create_zero_output_buffer(&client, np);
        let (cube_count, cube_dim) = calculate_launch_config(np);

        unsafe {
            exc_pol::lda_x_exc_pol::launch_unchecked::<CpuRuntime>(
                &client,
                cube_count,
                cube_dim,
                ArrayArg::from_raw_parts::<f64>(&rho_handle, 2, 1),
                ArrayArg::from_raw_parts::<f64>(&zk_handle, np, 1),
                ScalarArg::new(ALPHA),
                ScalarArg::new(DENS_THRESHOLD),
                ScalarArg::new(ZETA_THRESHOLD),
            )
            .unwrap();
        }

        let rust_zk = read_output_buffer(&client, zk_handle, np);

        let err = rel_err(rust_zk[0], c_zk[0]);
        assert!(
            err <= 1e-12,
            "asymmetric pol: rho=({:.2e},{:.2e}), rust={:.15e}, c={:.15e}, err={:.3e}",
            rho_a, rho_b, rust_zk[0], c_zk[0], err
        );
    }
    eprintln!("asymmetric spin polarization test passed");
}

/// Test asymmetric spins with all derivative orders.
/// vrho_a and vrho_b should differ for asymmetric spins.
#[test]
fn test_lda_x_pol_asymmetric_derivatives() {
    let rho_data = [0.999, 0.001, 0.9, 0.1, 0.5, 0.5];
    let np = 3; // 3 grid points

    let oracle = libxc_rs_verify::oracle_lda_all(LDA_X_ID, 2, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, np);
    let vrho_handle = create_zero_output_buffer(&client, np * 2);
    let v2rho2_handle = create_zero_output_buffer(&client, np * 3);
    let (cube_count, cube_dim) = calculate_launch_config(np);

    unsafe {
        fxc_pol::lda_x_fxc_pol::launch_unchecked::<CpuRuntime>(
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

    let rust_zk = read_output_buffer(&client, zk_handle, np);
    let rust_vrho = read_output_buffer(&client, vrho_handle, np * 2);
    let rust_v2rho2 = read_output_buffer(&client, v2rho2_handle, np * 3);

    for i in 0..np {
        let zk_err = rel_err(rust_zk[i], oracle.zk[i]);
        assert!(zk_err <= 1e-12, "asymmetric deriv zk[{i}]: err={zk_err:.3e}");

        for c in 0..2 {
            let idx = i * 2 + c;
            let vrho_err = rel_err(rust_vrho[idx], oracle.vrho[idx]);
            assert!(vrho_err <= 1e-10, "asymmetric deriv vrho[{idx}]: err={vrho_err:.3e}");
        }

        // For asymmetric spins (points 0 and 1), vrho_a != vrho_b
        if i < 2 {
            let vrho_a = rust_vrho[i * 2];
            let vrho_b = rust_vrho[i * 2 + 1];
            assert!(
                (vrho_a - vrho_b).abs() > 1e-10,
                "asymmetric spins should produce different vrho: vrho_a={vrho_a:.6e}, vrho_b={vrho_b:.6e}"
            );
        }

        for c in 0..3 {
            let idx = i * 3 + c;
            let v2_err = rel_err(rust_v2rho2[idx], oracle.v2rho2[idx]);
            assert!(v2_err <= 1e-8, "asymmetric deriv v2rho2[{idx}]: err={v2_err:.3e}");
        }
    }
    eprintln!("asymmetric spin derivative test passed");
}

// ============================================================================
// 5. SYMMETRIC POLARIZED == UNPOLARIZED SELF-CONSISTENCY
// ============================================================================

/// For rho_total, polarized (rho/2, rho/2) should give same zk as unpolarized rho.
#[test]
fn test_lda_x_pol_symmetric_matches_unpol() {
    let total_densities = [0.1, 1.0, 10.0, 100.0];

    for &rho_total in &total_densities {
        let client = cpu_client();

        // Unpolarized
        let unpol_rho = [rho_total];
        let rho_handle = create_input_buffer(&client, &unpol_rho);
        let zk_handle = create_zero_output_buffer(&client, 1);
        let (cc, cd) = calculate_launch_config(1);
        unsafe {
            exc_unpol::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
                &client, cc, cd,
                ArrayArg::from_raw_parts::<f64>(&rho_handle, 1, 1),
                ArrayArg::from_raw_parts::<f64>(&zk_handle, 1, 1),
                ScalarArg::new(ALPHA),
                ScalarArg::new(DENS_THRESHOLD),
                ScalarArg::new(ZETA_THRESHOLD),
            ).unwrap();
        }
        let unpol_zk = read_output_buffer(&client, zk_handle, 1);

        // Polarized with equal spins
        let pol_rho = [rho_total / 2.0, rho_total / 2.0];
        let rho_handle = create_input_buffer(&client, &pol_rho);
        let zk_handle = create_zero_output_buffer(&client, 1);
        let (cc, cd) = calculate_launch_config(1);
        unsafe {
            exc_pol::lda_x_exc_pol::launch_unchecked::<CpuRuntime>(
                &client, cc, cd,
                ArrayArg::from_raw_parts::<f64>(&rho_handle, 2, 1),
                ArrayArg::from_raw_parts::<f64>(&zk_handle, 1, 1),
                ScalarArg::new(ALPHA),
                ScalarArg::new(DENS_THRESHOLD),
                ScalarArg::new(ZETA_THRESHOLD),
            ).unwrap();
        }
        let pol_zk = read_output_buffer(&client, zk_handle, 1);

        let err = if unpol_zk[0].abs() < 1e-30 {
            (unpol_zk[0] - pol_zk[0]).abs()
        } else {
            ((unpol_zk[0] - pol_zk[0]) / unpol_zk[0]).abs()
        };

        assert!(
            err <= 1e-14,
            "symmetric pol vs unpol: rho={:.2e}, unpol_zk={:.15e}, pol_zk={:.15e}, err={:.3e}",
            rho_total, unpol_zk[0], pol_zk[0], err
        );
    }
    eprintln!("symmetric pol matches unpol test passed");
}

// ============================================================================
// 6. LARGE BATCH TEST
// ============================================================================

/// Test 10000-point batch to verify multi-workgroup dispatch.
/// CubeCount = ceil(10000/256) = 40 workgroups; last workgroup has
/// 10000 - 39*256 = 16 active threads out of 256.
#[test]
fn test_lda_x_large_batch() {
    let n = 10000;
    let rho_data: Vec<f64> = (0..n)
        .map(|i| 0.01 + (100.0 - 0.01) * (i as f64) / ((n - 1) as f64))
        .collect();

    let c_zk = oracle_lda_exc(LDA_X_ID, 1, &rho_data).unwrap();

    let client = cpu_client();
    let rho_handle = create_input_buffer(&client, &rho_data);
    let zk_handle = create_zero_output_buffer(&client, n);
    let (cube_count, cube_dim) = calculate_launch_config(n);

    // Verify launch config: ceil(10000/256) = 40 workgroups
    match cube_count {
        CubeCount::Static(x, _, _) => {
            assert_eq!(x, 40, "expected 40 workgroups for 10000 points, got {x}");
        }
        _ => panic!("unexpected dynamic CubeCount"),
    }

    unsafe {
        exc_unpol::lda_x_exc_unpol::launch_unchecked::<CpuRuntime>(
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

    assert_eq!(rust_zk.len(), n, "output length mismatch");

    let mut max_err = 0.0f64;
    let mut nan_count = 0;
    let mut inf_count = 0;

    for i in 0..n {
        if rust_zk[i].is_nan() {
            nan_count += 1;
        }
        if rust_zk[i].is_infinite() {
            inf_count += 1;
        }
        let err = rel_err(rust_zk[i], c_zk[i]);
        max_err = max_err.max(err);
    }

    assert_eq!(nan_count, 0, "found {nan_count} NaN values in 10000-point batch");
    assert_eq!(inf_count, 0, "found {inf_count} Inf values in 10000-point batch");
    assert!(
        max_err <= 1e-12,
        "10000-point batch max relative error: {max_err:.3e} (limit 1e-12)"
    );

    // Extra check: last few points (in partial workgroup) should also match
    for i in (n - 20)..n {
        let err = rel_err(rust_zk[i], c_zk[i]);
        assert!(
            err <= 1e-12,
            "partial workgroup point {i}: rust={:.15e}, c={:.15e}, err={:.3e}",
            rust_zk[i], c_zk[i], err
        );
    }
    eprintln!("large batch (10000 points, 40 workgroups) test passed, max err: {max_err:.3e}");
}
