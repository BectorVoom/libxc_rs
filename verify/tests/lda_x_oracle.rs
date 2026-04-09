use libxc_rs_verify::oracle_lda_exc;

/// Test LDA_X (Slater exchange) in unpolarized mode against C libxc oracle.
///
/// LDA_X (Slater exchange): exc = -3/4 * (3/pi)^(1/3) * rho^(1/3)
/// For rho = 1.0: exc = -0.738558766382022...
#[test]
fn test_lda_x_unpolarized_oracle() {
    // XC_LDA_X = 1, XC_UNPOLARIZED = 1
    let rho = vec![0.1, 0.5, 1.0, 5.0, 10.0];
    let exc = oracle_lda_exc(1, 1, &rho).unwrap();

    // Verify we got results (5 points)
    assert_eq!(exc.len(), rho.len());

    // LDA exchange energy should be negative for positive densities
    for &e in &exc {
        assert!(e < 0.0, "LDA_X energy should be negative, got {e}");
    }

    // Verify against known analytical value for rho = 1.0
    let expected_rho1 = -0.738558766382022;
    approx::assert_relative_eq!(exc[2], expected_rho1, max_relative = 1e-12);
}

/// Test LDA_X in polarized mode (spin up + spin down).
///
/// Polarized mode uses interleaved rho = [rho_up, rho_down] pairs.
#[test]
fn test_lda_x_polarized_oracle() {
    // XC_POLARIZED = 2, rho = [rho_up, rho_down] interleaved
    let rho = vec![0.3, 0.2, 0.5, 0.5, 1.0, 0.0];
    let exc = oracle_lda_exc(1, 2, &rho).unwrap();

    // 3 grid points
    assert_eq!(exc.len(), 3);

    // All should be negative (or zero for zero total density, but rho_up+rho_down > 0 here)
    for &e in &exc {
        assert!(e < 0.0, "LDA_X energy should be negative, got {e}");
    }
}
