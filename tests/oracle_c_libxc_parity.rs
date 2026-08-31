use libxc_rs::{
    lookup_by_id, lookup_by_name, DerivativeOrder, EvaluationWorkspace, Functional,
    GgaInput, GgaOutput, LdaInput, LdaOutput, MggaInput, MggaOutput, Spin,
};
use libxc_rs_verify::{
    oracle_gga_all, oracle_lda_all, oracle_mgga_all,
};

const TIER1_CORPUS: &[&str] = &[
    "lda_x",
    "lda_c_pw",
    "lda_c_vwn",
    "lda_c_vwn_rpa",
    "gga_x_pbe",
    "gga_c_pbe",
    "gga_x_b88",
    "gga_c_lyp",
    "mgga_x_tpss",
];

const REMEDIATION_V4_CORPUS: &[&str] = &[
    // V4-01: _init scraped defaults
    "gga_k_llp",
    "gga_k_apbe",
    "gga_k_tw1",
    "gga_c_zpbeint",
    "gga_x_vmt_pbe",
    "lda_xc_1d_ehwlrg_1",
    // V4-02: Ported custom setter transforms
    "gga_k_absp1",
    "gga_x_mpw91",
    "gga_x_lambda_ch_n",
    "hyb_gga_x_s12h",
    "hyb_gga_x_lcgau",
    // "lda_xc_ksdt",
    // V4-03: Param name aliases and array normalizations
    "gga_k_pg1",
    "hyb_gga_xc_wb97x",
    "hyb_mgga_xc_gas22",
    "hyb_mgga_x_m06_sx",
    "gga_x_s12g",
    // V4-05: Regex fix for hcth_93
    "gga_xc_hcth_93",
];

#[test]
fn test_tier1_corpus_parity_against_c_libxc() {
    let np = 8;
    // Unpolarized densities
    let rho_unpol = vec![0.05, 0.1, 0.2, 0.4, 0.8, 1.2, 2.0, 5.0];
    let sigma_unpol = vec![0.002, 0.01, 0.04, 0.16, 0.64, 1.44, 4.0, 25.0];
    let lapl_unpol = vec![0.01, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0];
    let tau_unpol = vec![0.02, 0.06, 0.15, 0.35, 0.9, 1.8, 4.2, 12.0];

    // Polarized densities (interleaved a, b)
    let mut rho_pol = Vec::with_capacity(np * 2);
    let mut sigma_pol = Vec::with_capacity(np * 3);
    let mut lapl_pol = Vec::with_capacity(np * 2);
    let mut tau_pol = Vec::with_capacity(np * 2);
    for i in 0..np {
        let ra = rho_unpol[i] * 0.6;
        let rb = rho_unpol[i] * 0.4;
        rho_pol.push(ra);
        rho_pol.push(rb);

        let sa = sigma_unpol[i] * 0.36;
        let sab = sigma_unpol[i] * 0.24;
        let sb = sigma_unpol[i] * 0.16;
        sigma_pol.push(sa);
        sigma_pol.push(sab);
        sigma_pol.push(sb);

        lapl_pol.push(lapl_unpol[i] * 0.6);
        lapl_pol.push(lapl_unpol[i] * 0.4);

        tau_pol.push(tau_unpol[i] * 0.6);
        tau_pol.push(tau_unpol[i] * 0.4);
    }

    for &name in TIER1_CORPUS {
        let id = lookup_by_name(name).unwrap_or_else(|_| panic!("Failed to lookup {name}"));
        let meta = lookup_by_id(id.raw()).unwrap_or_else(|_| panic!("Failed to lookup meta for {name}"));

        for spin in [Spin::Unpolarized, Spin::Polarized] {
            let spin_int = match spin {
                Spin::Unpolarized => 1,
                Spin::Polarized => 2,
            };

            let func = Functional::new(id, spin)
                .unwrap_or_else(|e| panic!("Failed to construct {name} with spin {spin:?}: {e}"));
            let mut ws = EvaluationWorkspace::new(np, spin);

            match meta.family {
                libxc_rs::Family::Lda => {
                    let rho = if spin == Spin::Unpolarized { &rho_unpol } else { &rho_pol };
                    let input = LdaInput::new(rho, np, spin).unwrap();
                    let c_res = oracle_lda_all(id.raw() as i32, spin_int, rho)
                        .unwrap_or_else(|e| panic!("C libxc LDA failed for {name}: {e}"));

                    let mut zk = vec![0.0; np];
                    let dim_vrho = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let mut vrho = vec![0.0; np * dim_vrho];

                    let mut output = LdaOutput::new(
                        Some(&mut zk),
                        Some(&mut vrho),
                        None,
                        None,
                        None,
                        np,
                        spin,
                    ).unwrap();

                    func.evaluate_lda(&input, DerivativeOrder::Vxc, &mut output, &mut ws)
                        .unwrap_or_else(|e| panic!("evaluate_lda failed for {name}: {e}"));

                    for i in 0..np {
                        let diff_zk = (zk[i] - c_res.zk[i]).abs();
                        let max_zk = zk[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                        let rel_zk = diff_zk / max_zk;
                        assert!(
                            rel_zk <= 1e-14 || diff_zk <= 1e-15,
                            "ZK mismatch in {name} (spin {spin:?}) at point {i}: rust={}, C={}, rel={rel_zk}",
                            zk[i], c_res.zk[i]
                        );
                    }

                    for i in 0..(np * dim_vrho) {
                        let diff_vrho = (vrho[i] - c_res.vrho[i]).abs();
                        let max_vrho = vrho[i].abs().max(c_res.vrho[i].abs()).max(1e-15);
                        let rel_vrho = diff_vrho / max_vrho;
                        assert!(
                            rel_vrho <= 1e-14 || diff_vrho <= 1e-15,
                            "VRHO mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vrho}",
                            vrho[i], c_res.vrho[i]
                        );
                    }
                }
                libxc_rs::Family::Gga => {
                    let rho = if spin == Spin::Unpolarized { &rho_unpol } else { &rho_pol };
                    let sigma = if spin == Spin::Unpolarized { &sigma_unpol } else { &sigma_pol };
                    let input = GgaInput::new(rho, sigma, np, spin).unwrap();
                    let c_res = oracle_gga_all(id.raw() as i32, spin_int, rho, sigma)
                        .unwrap_or_else(|e| panic!("C libxc GGA failed for {name}: {e}"));

                    let mut zk = vec![0.0; np];
                    let dim_vrho = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let dim_vsigma = if spin == Spin::Unpolarized { 1 } else { 3 };
                    let mut vrho = vec![0.0; np * dim_vrho];
                    let mut vsigma = vec![0.0; np * dim_vsigma];

                    let mut output = GgaOutput::new(
                        Some(&mut zk),
                        Some(&mut vrho),
                        Some(&mut vsigma),
                        None, None, None,
                        None, None, None, None,
                        None, None, None, None, None,
                        np,
                        spin,
                    ).unwrap();

                    func.evaluate_gga(&input, DerivativeOrder::Vxc, &mut output, &mut ws)
                        .unwrap_or_else(|e| panic!("evaluate_gga failed for {name}: {e}"));

                    for i in 0..np {
                        let diff_zk = (zk[i] - c_res.zk[i]).abs();
                        let max_zk = zk[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                        let rel_zk = diff_zk / max_zk;
                        assert!(
                            rel_zk <= 1e-14 || diff_zk <= 1e-15,
                            "ZK mismatch in {name} (spin {spin:?}) at point {i}: rust={}, C={}, rel={rel_zk}",
                            zk[i], c_res.zk[i]
                        );
                    }

                    for i in 0..(np * dim_vrho) {
                        let diff_vrho = (vrho[i] - c_res.vrho[i]).abs();
                        let max_vrho = vrho[i].abs().max(c_res.vrho[i].abs()).max(1e-15);
                        let rel_vrho = diff_vrho / max_vrho;
                        assert!(
                            rel_vrho <= 1e-14 || diff_vrho <= 1e-15,
                            "VRHO mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vrho}",
                            vrho[i], c_res.vrho[i]
                        );
                    }

                    for i in 0..(np * dim_vsigma) {
                        let diff_vsigma = (vsigma[i] - c_res.vsigma[i]).abs();
                        let max_vsigma = vsigma[i].abs().max(c_res.vsigma[i].abs()).max(1e-15);
                        let rel_vsigma = diff_vsigma / max_vsigma;
                        assert!(
                            rel_vsigma <= 1e-14 || diff_vsigma <= 1e-15,
                            "VSIGMA mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vsigma}",
                            vsigma[i], c_res.vsigma[i]
                        );
                    }
                }
                libxc_rs::Family::Mgga => {
                    let rho = if spin == Spin::Unpolarized { &rho_unpol } else { &rho_pol };
                    let sigma = if spin == Spin::Unpolarized { &sigma_unpol } else { &sigma_pol };
                    let lapl = if spin == Spin::Unpolarized { &lapl_unpol } else { &lapl_pol };
                    let tau = if spin == Spin::Unpolarized { &tau_unpol } else { &tau_pol };

                    let input = MggaInput::new(rho, sigma, lapl, tau, np, spin).unwrap();
                    let c_res = oracle_mgga_all(id.raw() as i32, spin_int, rho, sigma, lapl, tau)
                        .unwrap_or_else(|e| panic!("C libxc MGGA failed for {name}: {e}"));

                    let mut zk = vec![0.0; np];
                    let dim_vrho = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let dim_vsigma = if spin == Spin::Unpolarized { 1 } else { 3 };
                    let dim_vlapl = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let dim_vtau = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let mut vrho = vec![0.0; np * dim_vrho];
                    let mut vsigma = vec![0.0; np * dim_vsigma];
                    let mut vlapl = vec![0.0; np * dim_vlapl];
                    let mut vtau = vec![0.0; np * dim_vtau];

                    let mut output = MggaOutput::default();
                    output.zk = Some(&mut zk);
                    output.vrho = Some(&mut vrho);
                    output.vsigma = Some(&mut vsigma);
                    output.vlapl = Some(&mut vlapl);
                    output.vtau = Some(&mut vtau);

                    func.evaluate_mgga(&input, DerivativeOrder::Vxc, &mut output, &mut ws)
                        .unwrap_or_else(|e| panic!("evaluate_mgga failed for {name}: {e}"));

                    for i in 0..np {
                        let diff_zk = (zk[i] - c_res.zk[i]).abs();
                        let max_zk = zk[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                        let rel_zk = diff_zk / max_zk;
                        assert!(
                            rel_zk <= 1e-14 || diff_zk <= 1e-15,
                            "ZK mismatch in {name} (spin {spin:?}) at point {i}: rust={}, C={}, rel={rel_zk}",
                            zk[i], c_res.zk[i]
                        );
                    }

                    for i in 0..(np * dim_vrho) {
                        let diff_vrho = (vrho[i] - c_res.vrho[i]).abs();
                        let max_vrho = vrho[i].abs().max(c_res.vrho[i].abs()).max(1e-15);
                        let rel_vrho = diff_vrho / max_vrho;
                        assert!(
                            rel_vrho <= 1e-14 || diff_vrho <= 1e-15,
                            "VRHO mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vrho}",
                            vrho[i], c_res.vrho[i]
                        );
                    }

                    for i in 0..(np * dim_vsigma) {
                        let diff_vsigma = (vsigma[i] - c_res.vsigma[i]).abs();
                        let max_vsigma = vsigma[i].abs().max(c_res.vsigma[i].abs()).max(1e-15);
                        let rel_vsigma = diff_vsigma / max_vsigma;
                        assert!(
                            rel_vsigma <= 1e-14 || diff_vsigma <= 1e-15,
                            "VSIGMA mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vsigma}",
                            vsigma[i], c_res.vsigma[i]
                        );
                    }

                    for i in 0..(np * dim_vlapl) {
                        let diff_vlapl = (vlapl[i] - c_res.vlapl[i]).abs();
                        let max_vlapl = vlapl[i].abs().max(c_res.vlapl[i].abs()).max(1e-15);
                        let rel_vlapl = diff_vlapl / max_vlapl;
                        assert!(
                            rel_vlapl <= 1e-14 || diff_vlapl <= 1e-15,
                            "VLAPL mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vlapl}",
                            vlapl[i], c_res.vlapl[i]
                        );
                    }

                    for i in 0..(np * dim_vtau) {
                        let diff_tau = (vtau[i] - c_res.vtau[i]).abs();
                        let max_tau = vtau[i].abs().max(c_res.vtau[i].abs()).max(1e-15);
                        let rel_tau = diff_tau / max_tau;
                        assert!(
                            rel_tau <= 1e-14 || diff_tau <= 1e-15,
                            "VTAU mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_tau}",
                            vtau[i], c_res.vtau[i]
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_remediation_v4_corpus_parity_against_c_libxc() {
    let np = 8;
    let thresholds = libxc_core::model::Thresholds::default();

    // Unpolarized densities
    let rho_unpol = vec![0.05, 0.1, 0.2, 0.4, 0.8, 1.2, 2.0, 5.0];
    let sigma_unpol = vec![0.002, 0.01, 0.04, 0.16, 0.64, 1.44, 4.0, 25.0];
    let lapl_unpol = vec![0.01, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0];
    let tau_unpol = vec![0.02, 0.06, 0.15, 0.35, 0.9, 1.8, 4.2, 12.0];

    // Polarized densities (interleaved a, b)
    let mut rho_pol = Vec::with_capacity(np * 2);
    let mut sigma_pol = Vec::with_capacity(np * 3);
    let mut lapl_pol = Vec::with_capacity(np * 2);
    let mut tau_pol = Vec::with_capacity(np * 2);
    for i in 0..np {
        let ra = rho_unpol[i] * 0.6;
        let rb = rho_unpol[i] * 0.4;
        rho_pol.push(ra);
        rho_pol.push(rb);

        let sa = sigma_unpol[i] * 0.36;
        let sab = sigma_unpol[i] * 0.24;
        let sb = sigma_unpol[i] * 0.16;
        sigma_pol.push(sa);
        sigma_pol.push(sab);
        sigma_pol.push(sb);

        lapl_pol.push(lapl_unpol[i] * 0.6);
        lapl_pol.push(lapl_unpol[i] * 0.4);

        tau_pol.push(tau_unpol[i] * 0.6);
        tau_pol.push(tau_unpol[i] * 0.4);
    }

    for &name in REMEDIATION_V4_CORPUS {
        println!("Checking parity for {name}...");
        let id = lookup_by_name(name).unwrap_or_else(|_| panic!("Failed to lookup {name}"));
        let meta = lookup_by_id(id.raw()).unwrap_or_else(|_| panic!("Failed to lookup meta for {name}"));

        for spin in [Spin::Unpolarized, Spin::Polarized] {
            let spin_int = match spin {
                Spin::Unpolarized => 1,
                Spin::Polarized => 2,
            };

            match meta.family {
                libxc_rs::Family::Lda => {
                    let rho = if spin == Spin::Unpolarized { &rho_unpol } else { &rho_pol };
                    let input = LdaInput::new(rho, np, spin).unwrap();
                    let c_res = oracle_lda_all(id.raw() as i32, spin_int, rho)
                        .unwrap_or_else(|e| panic!("C libxc LDA failed for {name}: {e}"));

                    let mut zk = vec![0.0; np];
                    let dim_vrho = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let mut vrho = vec![0.0; np * dim_vrho];

                    let mut output = LdaOutput::new(
                        Some(&mut zk),
                        Some(&mut vrho),
                        None,
                        None,
                        None,
                        np,
                        spin,
                    ).unwrap();

                    libxc_reval::routing::dispatch_lda_by_name(
                        name,
                        &input,
                        &mut output,
                        DerivativeOrder::Vxc,
                        spin,
                        &thresholds,
                    )
                    .unwrap_or_else(|| panic!("failed to dispatch LDA {name}"))
                    .unwrap_or_else(|e| panic!("LDA eval error for {name}: {e}"));

                    for i in 0..np {
                        let diff_zk = (zk[i] - c_res.zk[i]).abs();
                        let max_zk = zk[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                        let rel_zk = diff_zk / max_zk;
                        assert!(
                            rel_zk <= 1e-12 || diff_zk <= 1e-12,
                            "ZK mismatch in {name} (spin {spin:?}) at point {i}: rust={}, C={}, rel={rel_zk}",
                            zk[i], c_res.zk[i]
                        );
                    }

                    for i in 0..(np * dim_vrho) {
                        let diff_vrho = (vrho[i] - c_res.vrho[i]).abs();
                        let max_vrho = vrho[i].abs().max(c_res.vrho[i].abs()).max(1e-15);
                        let rel_vrho = diff_vrho / max_vrho;
                        assert!(
                            rel_vrho <= 1e-12 || diff_vrho <= 1e-12,
                            "VRHO mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vrho}",
                            vrho[i], c_res.vrho[i]
                        );
                    }
                }
                libxc_rs::Family::Gga => {
                    let rho = if spin == Spin::Unpolarized { &rho_unpol } else { &rho_pol };
                    let sigma = if spin == Spin::Unpolarized { &sigma_unpol } else { &sigma_pol };
                    let input = GgaInput::new(rho, sigma, np, spin).unwrap();
                    let c_res = oracle_gga_all(id.raw() as i32, spin_int, rho, sigma)
                        .unwrap_or_else(|e| panic!("C libxc GGA failed for {name}: {e}"));

                    let mut zk = vec![0.0; np];
                    let dim_vrho = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let dim_vsigma = if spin == Spin::Unpolarized { 1 } else { 3 };
                    let mut vrho = vec![0.0; np * dim_vrho];
                    let mut vsigma = vec![0.0; np * dim_vsigma];

                    let mut output = GgaOutput::new(
                        Some(&mut zk),
                        Some(&mut vrho),
                        Some(&mut vsigma),
                        None, None, None,
                        None, None, None, None,
                        None, None, None, None, None,
                        np,
                        spin,
                    ).unwrap();

                    libxc_reval::routing::dispatch_gga_by_name(
                        name,
                        &input,
                        &mut output,
                        DerivativeOrder::Vxc,
                        spin,
                        &thresholds,
                    )
                    .unwrap_or_else(|| panic!("failed to dispatch GGA {name}"))
                    .unwrap_or_else(|e| panic!("GGA eval error for {name}: {e}"));

                    for i in 0..np {
                        let diff_zk = (zk[i] - c_res.zk[i]).abs();
                        let max_zk = zk[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                        let rel_zk = diff_zk / max_zk;
                        assert!(
                            rel_zk <= 1e-12 || diff_zk <= 1e-12,
                            "ZK mismatch in {name} (spin {spin:?}) at point {i}: rust={}, C={}, rel={rel_zk}",
                            zk[i], c_res.zk[i]
                        );
                    }

                    for i in 0..(np * dim_vrho) {
                        let diff_vrho = (vrho[i] - c_res.vrho[i]).abs();
                        let max_vrho = vrho[i].abs().max(c_res.vrho[i].abs()).max(1e-15);
                        let rel_vrho = diff_vrho / max_vrho;
                        assert!(
                            rel_vrho <= 1e-12 || diff_vrho <= 1e-12,
                            "VRHO mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vrho}",
                            vrho[i], c_res.vrho[i]
                        );
                    }

                    for i in 0..(np * dim_vsigma) {
                        let diff_vsigma = (vsigma[i] - c_res.vsigma[i]).abs();
                        let max_vsigma = vsigma[i].abs().max(c_res.vsigma[i].abs()).max(1e-15);
                        let rel_vsigma = diff_vsigma / max_vsigma;
                        assert!(
                            rel_vsigma <= 1e-12 || diff_vsigma <= 1e-12,
                            "VSIGMA mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vsigma}",
                            vsigma[i], c_res.vsigma[i]
                        );
                    }
                }
                libxc_rs::Family::Mgga => {
                    let rho = if spin == Spin::Unpolarized { &rho_unpol } else { &rho_pol };
                    let sigma = if spin == Spin::Unpolarized { &sigma_unpol } else { &sigma_pol };
                    let lapl = if spin == Spin::Unpolarized { &lapl_unpol } else { &lapl_pol };
                    let tau = if spin == Spin::Unpolarized { &tau_unpol } else { &tau_pol };

                    let input = MggaInput::new(rho, sigma, lapl, tau, np, spin).unwrap();
                    let c_res = oracle_mgga_all(id.raw() as i32, spin_int, rho, sigma, lapl, tau)
                        .unwrap_or_else(|e| panic!("C libxc MGGA failed for {name}: {e}"));

                    let mut zk = vec![0.0; np];
                    let dim_vrho = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let dim_vsigma = if spin == Spin::Unpolarized { 1 } else { 3 };
                    let dim_vlapl = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let dim_vtau = if spin == Spin::Unpolarized { 1 } else { 2 };
                    let mut vrho = vec![0.0; np * dim_vrho];
                    let mut vsigma = vec![0.0; np * dim_vsigma];
                    let mut vlapl = vec![0.0; np * dim_vlapl];
                    let mut vtau = vec![0.0; np * dim_vtau];

                    let mut output = MggaOutput::default();
                    output.zk = Some(&mut zk);
                    output.vrho = Some(&mut vrho);
                    output.vsigma = Some(&mut vsigma);
                    output.vlapl = Some(&mut vlapl);
                    output.vtau = Some(&mut vtau);

                    libxc_reval::routing::dispatch_mgga_by_name(
                        name,
                        &input,
                        &mut output,
                        DerivativeOrder::Vxc,
                        spin,
                        &thresholds,
                    )
                    .unwrap_or_else(|| panic!("failed to dispatch MGGA {name}"))
                    .unwrap_or_else(|e| panic!("MGGA eval error for {name}: {e}"));

                    for i in 0..np {
                        let diff_zk = (zk[i] - c_res.zk[i]).abs();
                        let max_zk = zk[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                        let rel_zk = diff_zk / max_zk;
                        assert!(
                            rel_zk <= 1e-12 || diff_zk <= 1e-12,
                            "ZK mismatch in {name} (spin {spin:?}) at point {i}: rust={}, C={}, rel={rel_zk}",
                            zk[i], c_res.zk[i]
                        );
                    }

                    for i in 0..(np * dim_vrho) {
                        let diff_vrho = (vrho[i] - c_res.vrho[i]).abs();
                        let max_vrho = vrho[i].abs().max(c_res.vrho[i].abs()).max(1e-15);
                        let rel_vrho = diff_vrho / max_vrho;
                        assert!(
                            rel_vrho <= 1e-12 || diff_vrho <= 1e-12,
                            "VRHO mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vrho}",
                            vrho[i], c_res.vrho[i]
                        );
                    }

                    for i in 0..(np * dim_vsigma) {
                        let diff_vsigma = (vsigma[i] - c_res.vsigma[i]).abs();
                        let max_vsigma = vsigma[i].abs().max(c_res.vsigma[i].abs()).max(1e-15);
                        let rel_vsigma = diff_vsigma / max_vsigma;
                        assert!(
                            rel_vsigma <= 1e-12 || diff_vsigma <= 1e-12,
                            "VSIGMA mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vsigma}",
                            vsigma[i], c_res.vsigma[i]
                        );
                    }

                    for i in 0..(np * dim_vlapl) {
                        let diff_vlapl = (vlapl[i] - c_res.vlapl[i]).abs();
                        let max_vlapl = vlapl[i].abs().max(c_res.vlapl[i].abs()).max(1e-15);
                        let rel_vlapl = diff_vlapl / max_vlapl;
                        assert!(
                            rel_vlapl <= 1e-12 || diff_vlapl <= 1e-12,
                            "VLAPL mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_vlapl}",
                            vlapl[i], c_res.vlapl[i]
                        );
                    }

                    for i in 0..(np * dim_vtau) {
                        let diff_tau = (vtau[i] - c_res.vtau[i]).abs();
                        let max_tau = vtau[i].abs().max(c_res.vtau[i].abs()).max(1e-15);
                        let rel_tau = diff_tau / max_tau;
                        assert!(
                            rel_tau <= 1e-12 || diff_tau <= 1e-12,
                            "VTAU mismatch in {name} (spin {spin:?}) at index {i}: rust={}, C={}, rel={rel_tau}",
                            vtau[i], c_res.vtau[i]
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn test_all_reachable_functionals_parity_against_c_libxc() {
    let np = 4;
    let rho_unpol = vec![0.1, 0.2, 0.5, 1.0];
    let sigma_unpol = vec![0.01, 0.04, 0.25, 1.0];
    let lapl_unpol = vec![0.05, 0.1, 0.5, 1.0];
    let tau_unpol = vec![0.06, 0.15, 0.9, 2.0];

    let mut verified_count = 0;

    for &(_fam, name) in libxc_reval::routing::SUPPORTED {
        let id = match lookup_by_name(name) {
            Ok(i) => i,
            Err(_) => continue,
        };
        let raw = id.raw();
        let meta = match lookup_by_id(raw) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let spin = Spin::Unpolarized;
        let func = match Functional::new(id, spin) {
            Ok(f) => f,
            Err(_) => continue,
        };

        let mut ws = EvaluationWorkspace::new(np, spin);
        let mut zk_rust = vec![0.0; np];

        match meta.family {
            libxc_rs::Family::Lda => {
                let input = LdaInput::new(&rho_unpol, np, spin).unwrap();
                let mut output = LdaOutput::new(
                    Some(&mut zk_rust),
                    None,
                    None,
                    None,
                    None,
                    np,
                    spin,
                ).unwrap();

                if func.evaluate_lda(&input, DerivativeOrder::Exc, &mut output, &mut ws).is_err() {
                    continue;
                }

                let c_res = match oracle_lda_all(raw as i32, 1, &rho_unpol) {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                let mut ok = true;
                for i in 0..np {
                    let diff = (zk_rust[i] - c_res.zk[i]).abs();
                    let max_val = zk_rust[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                    let rel = diff / max_val;
                    if rel > 1e-6 && diff > 1e-8 {
                        println!("ZK mismatch in {} (id {}) at point {i}: rust={}, C={}, rel={rel}", meta.name, raw, zk_rust[i], c_res.zk[i]);
                        ok = false;
                        break;
                    }
                }
                if ok {
                    verified_count += 1;
                }
            }
            libxc_rs::Family::Gga => {
                let input = GgaInput::new(&rho_unpol, &sigma_unpol, np, spin).unwrap();
                let mut output = GgaOutput::new(
                    Some(&mut zk_rust),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    np,
                    spin,
                ).unwrap();

                if func.evaluate_gga(&input, DerivativeOrder::Exc, &mut output, &mut ws).is_err() {
                    continue;
                }

                let c_res = match oracle_gga_all(raw as i32, 1, &rho_unpol, &sigma_unpol) {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                let mut ok = true;
                for i in 0..np {
                    let diff = (zk_rust[i] - c_res.zk[i]).abs();
                    let max_val = zk_rust[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                    let rel = diff / max_val;
                    if rel > 1e-6 && diff > 1e-8 {
                        println!("ZK mismatch in {} (id {}) at point {i}: rust={}, C={}, rel={rel}", meta.name, raw, zk_rust[i], c_res.zk[i]);
                        ok = false;
                        break;
                    }
                }
                if ok {
                    verified_count += 1;
                }
            }
            libxc_rs::Family::Mgga => {
                let input = MggaInput::new(&rho_unpol, &sigma_unpol, &lapl_unpol, &tau_unpol, np, spin).unwrap();
                let mut output = MggaOutput {
                    zk: Some(&mut zk_rust),
                    ..Default::default()
                };

                if func.evaluate_mgga(&input, DerivativeOrder::Exc, &mut output, &mut ws).is_err() {
                    continue;
                }

                let c_res = match oracle_mgga_all(raw as i32, 1, &rho_unpol, &sigma_unpol, &lapl_unpol, &tau_unpol) {
                    Ok(r) => r,
                    Err(_) => continue,
                };

                let mut ok = true;
                for i in 0..np {
                    let diff = (zk_rust[i] - c_res.zk[i]).abs();
                    let max_val = zk_rust[i].abs().max(c_res.zk[i].abs()).max(1e-15);
                    let rel = diff / max_val;
                    if rel > 1e-6 && diff > 1e-8 {
                        println!("ZK mismatch in {} (id {}) at point {i}: rust={}, C={}, rel={rel}", meta.name, raw, zk_rust[i], c_res.zk[i]);
                        ok = false;
                        break;
                    }
                }
                if ok {
                    verified_count += 1;
                }
            }
        }
    }

    println!("Swept parity for {verified_count} reachable functionals against C libxc");
    assert!(
        verified_count >= 475,
        "Expected at least 475 functionals verified against C libxc, got {verified_count}"
    );
}
