//! Hybrid functional classification + CAM/NLC/aux query API.
//!
//! Pure-Rust port of libxc's `xc_hyb_type` (hybrids.c:82-118) and the
//! query-shape of `xc_hyb_cam_coef` / `xc_hyb_exx_coef`. Adds the public
//! `CamCoefficients` / `NlcCoefficients` structs and the matching
//! `Functional` methods (`hybrid_type`, `exx_coefficient`, `cam_coefficients`,
//! `nlc_coefficients`, `auxiliary_functionals`, `mix_coefficients`).
//!
//! The Rust port and the snapshotted `meta.hybrid_type` agreement is
//! verified at test time (Task 1 unit tests + verify/tests/hybrid_type_oracle.rs)
//! across all 649 functional IDs; no runtime drift between the two paths.

use crate::functional::Functional;
use crate::meta::HybridTerm;
use crate::model::{HybridTermKind, HybridType};

/// CAM / range-separated hybrid coefficients.
///
/// - `omega`: range-separation parameter (units: 1/bohr)
/// - `alpha`: full-range HF exchange fraction
/// - `beta`: short-range HF exchange fraction
///
/// Mirrors the (omega, alpha, beta) triple returned by libxc's
/// `xc_hyb_cam_coef` (hybrids.c:132-157).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CamCoefficients {
    pub omega: f64,
    pub alpha: f64,
    pub beta: f64,
}

/// Non-local correlation coefficients (VV10-family).
///
/// `b` and `c` are the two scalar parameters of the VV10 NLC kernel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NlcCoefficients {
    pub b: f64,
    pub c: f64,
}

/// Classify a functional based on its hybrid terms — Rust port of
/// libxc's `xc_hyb_type()` (hybrids.c:82-118).
///
/// Pitfall 6 note: the single-term `XC_HYB_NONE` screened-GGA case is
/// handled at xtask snapshot time (Plan 05-01) by emitting `hybrid_terms: &[]`
/// for those functionals, so the `terms.is_empty()` branch below correctly
/// returns `Semilocal` without needing a `HybridTermKind::None` variant.
pub fn classify_hybrid(terms: &[HybridTerm]) -> HybridType {
    if terms.is_empty() {
        return HybridType::Semilocal;
    }

    if terms.len() == 1 {
        return match terms[0].kind {
            HybridTermKind::Fock       => HybridType::Hybrid,
            HybridTermKind::ErfSr      => HybridType::Cam,
            HybridTermKind::YukawaSr   => HybridType::CamYukawa,
            HybridTermKind::GaussianSr => HybridType::CamGaussian,
            // Single-term PT2 is unusual but not forbidden; treat as Mixture
            // so the call site does not surface a misleading classification.
            HybridTermKind::Pt2        => HybridType::Mixture,
        };
    }

    if terms.len() == 2 {
        return match (terms[0].kind, terms[1].kind) {
            (HybridTermKind::ErfSr,      HybridTermKind::Fock) => HybridType::Cam,
            (HybridTermKind::YukawaSr,   HybridTermKind::Fock) => HybridType::CamYukawa,
            (HybridTermKind::GaussianSr, HybridTermKind::Fock) => HybridType::CamGaussian,
            (HybridTermKind::Pt2,        HybridTermKind::Fock) => HybridType::DoubleHybrid,
            _                                                   => HybridType::Mixture,
        };
    }

    HybridType::Mixture
}

impl Functional {
    /// Returns the snapshotted hybrid classification for this functional.
    ///
    /// O(1) — reads the static `meta.hybrid_type` field. The agreement
    /// `classify_hybrid(meta.hybrid_terms) == meta.hybrid_type` is verified
    /// at test time across all 649 IDs.
    pub fn hybrid_type(&self) -> HybridType {
        self.meta.hybrid_type
    }

    /// EXX (Hartree-Fock exchange) fraction for the "pure Fock hybrid" case.
    ///
    /// Returns:
    /// - `Some(coefficient)` if `hybrid_type() == HybridType::Hybrid` (one
    ///   term, kind = Fock — e.g. B3LYP returns `Some(0.20)`)
    /// - `None` otherwise (semilocal, CAM/CAMY/CAMG, double-hybrid, mixture)
    ///
    /// Mirrors the single-term `XC_HYB_FOCK` branch of `xc_hyb_cam_coef`
    /// (hybrids.c:140) where `alpha = hyb_coeff[0]`.
    pub fn exx_coefficient(&self) -> Option<f64> {
        if self.hybrid_type() != HybridType::Hybrid {
            return None;
        }
        self.meta.hybrid_terms.first().map(|t| t.coefficient)
    }

    /// CAM / range-separated coefficients (omega, alpha, beta).
    ///
    /// Returns `Some(CamCoefficients)` for CAM/CAMY/CAMG functionals (one
    /// or two terms with at least one range-separated kind), or for the
    /// pure-Fock single-term case where `omega=0, alpha=coefficient, beta=0`.
    /// Returns `None` for semilocal and other non-CAM-shaped functionals.
    ///
    /// Mirrors `xc_hyb_cam_coef` (hybrids.c:132-157) verbatim.
    pub fn cam_coefficients(&self) -> Option<CamCoefficients> {
        let terms = self.meta.hybrid_terms;
        match terms.len() {
            1 => {
                let t = &terms[0];
                if t.kind == HybridTermKind::Fock {
                    Some(CamCoefficients { omega: 0.0, alpha: t.coefficient, beta: 0.0 })
                } else if matches!(
                    t.kind,
                    HybridTermKind::ErfSr | HybridTermKind::YukawaSr | HybridTermKind::GaussianSr
                ) {
                    Some(CamCoefficients { omega: t.omega, alpha: 0.0, beta: t.coefficient })
                } else {
                    None
                }
            }
            2 => {
                let t0 = &terms[0];
                let t1 = &terms[1];
                // libxc's xc_hyb_cam_coef takes hyb_omega[0], hyb_coeff[0]=beta,
                // hyb_coeff[1]=alpha for two-term hybrids (range-separated short-range
                // term first, full-range Fock term second).
                if matches!(
                    t0.kind,
                    HybridTermKind::ErfSr | HybridTermKind::YukawaSr | HybridTermKind::GaussianSr
                ) && t1.kind == HybridTermKind::Fock
                {
                    Some(CamCoefficients {
                        omega: t0.omega,
                        alpha: t1.coefficient,
                        beta: t0.coefficient,
                    })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Non-local correlation coefficients (VV10-family).
    ///
    /// Reads directly from `meta.nlc_params` populated by xtask in Plan 05-01.
    /// Returns `None` for non-NLC functionals.
    pub fn nlc_coefficients(&self) -> Option<NlcCoefficients> {
        self.meta.nlc_params.map(|(b, c)| NlcCoefficients { b, c })
    }

    /// Slice of eagerly-constructed auxiliary `Functional` instances.
    ///
    /// Empty for non-hybrid / non-mixed functionals. Length 1..=6 for
    /// hybrid and double-hybrid functionals (B3LYP has 4, mgga_c_b94_hyb
    /// has 2, etc.). Aligned with `mix_coefficients()`.
    pub fn auxiliary_functionals(&self) -> &[Functional] {
        &self.auxiliaries
    }

    /// Per-aux mixing coefficients aligned with `auxiliary_functionals()`.
    ///
    /// Empty for non-hybrid functionals; otherwise length matches
    /// `auxiliary_functionals().len()`.
    pub fn mix_coefficients(&self) -> &[f64] {
        &self.mix_coefficients
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::HybridTerm;
    use crate::model::{FunctionalId, HybridTermKind, HybridType, Spin};
    use crate::registry::{all_functional_ids, lookup_by_id};

    fn term(kind: HybridTermKind, coefficient: f64, omega: f64) -> HybridTerm {
        HybridTerm { kind, coefficient, omega }
    }

    // ── classify_hybrid unit tests ────────────────────────────────────

    #[test]
    fn classify_empty_terms_is_semilocal() {
        assert_eq!(classify_hybrid(&[]), HybridType::Semilocal);
    }

    #[test]
    fn classify_single_fock_is_hybrid() {
        let terms = [term(HybridTermKind::Fock, 0.20, 0.0)];
        assert_eq!(classify_hybrid(&terms), HybridType::Hybrid);
    }

    #[test]
    fn classify_single_erf_sr_is_cam() {
        let terms = [term(HybridTermKind::ErfSr, 0.65, 0.33)];
        assert_eq!(classify_hybrid(&terms), HybridType::Cam);
    }

    #[test]
    fn classify_single_yukawa_sr_is_camy() {
        let terms = [term(HybridTermKind::YukawaSr, 0.5, 0.3)];
        assert_eq!(classify_hybrid(&terms), HybridType::CamYukawa);
    }

    #[test]
    fn classify_single_gaussian_sr_is_camg() {
        let terms = [term(HybridTermKind::GaussianSr, 0.5, 0.3)];
        assert_eq!(classify_hybrid(&terms), HybridType::CamGaussian);
    }

    #[test]
    fn classify_two_term_erf_sr_then_fock_is_cam() {
        let terms = [
            term(HybridTermKind::ErfSr, 0.65, 0.33),
            term(HybridTermKind::Fock, 0.19, 0.0),
        ];
        assert_eq!(classify_hybrid(&terms), HybridType::Cam);
    }

    #[test]
    fn classify_two_term_yukawa_sr_then_fock_is_camy() {
        let terms = [
            term(HybridTermKind::YukawaSr, 0.46, 0.5),
            term(HybridTermKind::Fock, 0.0, 0.0),
        ];
        assert_eq!(classify_hybrid(&terms), HybridType::CamYukawa);
    }

    #[test]
    fn classify_two_term_gaussian_sr_then_fock_is_camg() {
        let terms = [
            term(HybridTermKind::GaussianSr, 0.5, 0.3),
            term(HybridTermKind::Fock, 0.5, 0.0),
        ];
        assert_eq!(classify_hybrid(&terms), HybridType::CamGaussian);
    }

    #[test]
    fn classify_two_term_pt2_then_fock_is_double_hybrid() {
        let terms = [
            term(HybridTermKind::Pt2, 0.27, 0.0),
            term(HybridTermKind::Fock, 0.53, 0.0),
        ];
        assert_eq!(classify_hybrid(&terms), HybridType::DoubleHybrid);
    }

    #[test]
    fn classify_two_term_unknown_pair_is_mixture() {
        let terms = [
            term(HybridTermKind::Fock, 0.5, 0.0),
            term(HybridTermKind::ErfSr, 0.5, 0.3),
        ];
        assert_eq!(classify_hybrid(&terms), HybridType::Mixture);
    }

    #[test]
    fn classify_three_or_more_terms_is_mixture() {
        let terms = [
            term(HybridTermKind::Fock, 0.5, 0.0),
            term(HybridTermKind::ErfSr, 0.5, 0.3),
            term(HybridTermKind::YukawaSr, 0.0, 0.0),
        ];
        assert_eq!(classify_hybrid(&terms), HybridType::Mixture);
    }

    #[test]
    fn classify_single_pt2_is_mixture() {
        let terms = [term(HybridTermKind::Pt2, 1.0, 0.0)];
        // Single-term PT2 is not in xc_hyb_type's branches; we return Mixture
        // so the call site doesn't get a misleading "Semilocal" or "Hybrid".
        assert_eq!(classify_hybrid(&terms), HybridType::Mixture);
    }

    // ── three-way (Rust port == snapshot) sweep across all 649 ────────

    #[test]
    fn rust_port_matches_snapshot_all_649() {
        let mut mismatches: Vec<(u16, &'static str, HybridType, HybridType)> = Vec::new();
        let mut count = 0usize;
        for id in all_functional_ids() {
            count += 1;
            let meta = lookup_by_id(id.raw()).unwrap();
            let rust_port = classify_hybrid(meta.hybrid_terms);
            let snapshot = meta.hybrid_type;
            if rust_port != snapshot {
                mismatches.push((id.raw(), meta.name, rust_port, snapshot));
            }
        }
        assert_eq!(count, 649, "registry should contain 649 functionals");
        assert!(
            mismatches.is_empty(),
            "Rust port and meta.hybrid_type disagree on {} functionals: {:?}",
            mismatches.len(),
            mismatches
        );
    }

    // ── Functional method behavior tests ─────────────────────────────

    #[test]
    fn lda_x_exx_coefficient_is_none() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // LDA_X is Semilocal; exx_coefficient must be None.
        assert!(f.exx_coefficient().is_none());
    }

    #[test]
    fn lda_x_cam_coefficients_is_none() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // LDA_X is Semilocal; cam_coefficients must be None.
        assert!(f.cam_coefficients().is_none());
    }

    #[test]
    fn lda_x_nlc_coefficients_is_none() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // LDA_X has no NLC; nlc_coefficients must be None.
        assert!(f.nlc_coefficients().is_none());
    }

    #[test]
    fn semilocal_functional_has_empty_aux() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert!(f.auxiliary_functionals().is_empty());
        assert!(f.mix_coefficients().is_empty());
    }

    #[test]
    fn hybrid_type_reads_meta_field() {
        // hybrid_type() must return the meta.hybrid_type snapshot. While the
        // current generated.rs leaves all functionals as Semilocal (Plan 05-01
        // metadata population deferred), this still exercises the read path.
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert_eq!(f.hybrid_type(), f.meta.hybrid_type);
    }
}
