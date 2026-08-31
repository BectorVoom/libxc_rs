//! LdaFunctional enum: enumerates the compiled LDA functionals routable
//! by `dispatch_lda`.
//!
//! Each variant maps one or more libxc functional IDs to a kernel module in
//! `crates/kernel-lda`. Variants are distinct when functionals share kernel
//! source but use different default ext-param values
//! (e.g. `LdaXc1dEhwlrg1/2/3` all dispatch to the `lda_xc_1d_ehwlrg` kernel
//! with different defaults).
//!
//! Deferred functionals (lda_c_pk09=554, lda_xc_ksdt=259, lda_c_pw_erf=654,
//! lda_c_pmgb06=590) are NOT represented here — `LdaFunctional::from_id`
//! returns `Err(UnsupportedFunctional)` for them, matching the `is_deferred`
//! helper in `crate::deferred::lda`.
//!
//! `lda_k_gds08_worker` (libxc internal id 100001) is NOT represented either:
//! it is an internal building block not registered in libxc_rs's u16 ID space.
//! Its kernel module is still compiled and reusable from inside the crate,
//! but it cannot be reached via `FunctionalId::from_raw`.

use crate::error::LibxcRsError;
use crate::model::FunctionalId;

/// Enumerates the 38 compiled LDA functionals dispatch_lda can route.
///
/// Variants for `LdaXc1dEhwlrg{1,2,3}` share one kernel module
/// (`lda_xc_1d_ehwlrg`) but use different default parameter values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LdaFunctional {
    HybLdaXcBn05,        // 588
    LdaC1dCsc,           // 18
    LdaC1dLoos,          // 26
    LdaC2dAmgb,          // 15
    LdaC2dPrm,           // 16
    LdaCChachiyo,        // 287
    LdaCChachiyoMod,     // 307
    LdaCGk72,            // 578
    LdaCGombas,          // 24
    LdaCHl,              // 4
    LdaCLp96,            // 289
    LdaCMl1,             // 22
    LdaCPw,              // 12
    LdaCPz,              // 9
    LdaCRc04,            // 27
    LdaCRpa,             // 3
    LdaCVwn,             // 7
    LdaCVwn1,            // 28
    LdaCVwn2,            // 29
    LdaCVwn3,            // 30
    LdaCVwn4,            // 31
    LdaCVwnRpa,          // 8
    LdaCW20,             // 317
    LdaCWigner,          // 2
    LdaKTf,              // 50
    LdaKZlp,             // 550
    LdaX,                // 1
    LdaX2d,              // 19
    LdaXErf,             // 546
    LdaXRel,             // 532
    LdaXSloc,            // 692
    LdaXYukawa,          // 641
    LdaXc1dEhwlrg1,      // 536 (kernel module: lda_xc_1d_ehwlrg)
    LdaXc1dEhwlrg2,      // 537 (kernel module: lda_xc_1d_ehwlrg)
    LdaXc1dEhwlrg3,      // 538 (kernel module: lda_xc_1d_ehwlrg)
    LdaXcTeter93,        // 20
    LdaXcTih,            // 599  (vxc-only special case — has_exc() == false)
    LdaXcZlp,            // 43
}

impl LdaFunctional {
    /// Map a libxc functional ID to a dispatchable LDA variant, or return an error.
    ///
    /// Returns `Err(UnsupportedFunctional)` for:
    /// - Any of the 4 deferred IDs (lda_c_pk09=554, lda_xc_ksdt=259,
    ///   lda_c_pw_erf=654, lda_c_pmgb06=590)
    /// - Any LDA-family ID that is not present in `crates/kernel-lda` (e.g.
    ///   `lda_x_rae`, `lda_c_xalpha`)
    /// - Any non-LDA ID (these should be routed via dispatch_gga / dispatch_mgga)
    pub fn from_id(id: FunctionalId) -> Result<Self, LibxcRsError> {
        // Reject deferred IDs explicitly so the error message is actionable.
        if crate::deferred::lda::is_deferred(id.raw()) {
            return Err(LibxcRsError::UnsupportedFunctional {
                id,
                reason: "LDA functional is tracked as deferred (CubeCL proc-macro stack limit). \
                         See crates/libxc-core/src/deferred.rs",
            });
        }
        match id.raw() {
            1   => Ok(Self::LdaX),
            2   => Ok(Self::LdaCWigner),
            3   => Ok(Self::LdaCRpa),
            4   => Ok(Self::LdaCHl),
            7   => Ok(Self::LdaCVwn),
            8   => Ok(Self::LdaCVwnRpa),
            9   => Ok(Self::LdaCPz),
            12  => Ok(Self::LdaCPw),
            15  => Ok(Self::LdaC2dAmgb),
            16  => Ok(Self::LdaC2dPrm),
            18  => Ok(Self::LdaC1dCsc),
            19  => Ok(Self::LdaX2d),
            20  => Ok(Self::LdaXcTeter93),
            22  => Ok(Self::LdaCMl1),
            24  => Ok(Self::LdaCGombas),
            26  => Ok(Self::LdaC1dLoos),
            27  => Ok(Self::LdaCRc04),
            28  => Ok(Self::LdaCVwn1),
            29  => Ok(Self::LdaCVwn2),
            30  => Ok(Self::LdaCVwn3),
            31  => Ok(Self::LdaCVwn4),
            43  => Ok(Self::LdaXcZlp),
            50  => Ok(Self::LdaKTf),
            287 => Ok(Self::LdaCChachiyo),
            289 => Ok(Self::LdaCLp96),
            307 => Ok(Self::LdaCChachiyoMod),
            317 => Ok(Self::LdaCW20),
            532 => Ok(Self::LdaXRel),
            536 => Ok(Self::LdaXc1dEhwlrg1),
            537 => Ok(Self::LdaXc1dEhwlrg2),
            538 => Ok(Self::LdaXc1dEhwlrg3),
            546 => Ok(Self::LdaXErf),
            550 => Ok(Self::LdaKZlp),
            578 => Ok(Self::LdaCGk72),
            588 => Ok(Self::HybLdaXcBn05),
            599 => Ok(Self::LdaXcTih),
            641 => Ok(Self::LdaXYukawa),
            692 => Ok(Self::LdaXSloc),
            _ => Err(LibxcRsError::UnsupportedFunctional {
                id,
                reason: "LDA functional has no typed enum variant in LdaFunctional; use Functional::new or dispatch_lda_by_id",
            }),
        }
    }

    /// Reverse of `from_id`: return the canonical libxc functional ID for
    /// this variant.
    pub fn to_id(self) -> FunctionalId {
        let raw = match self {
            Self::HybLdaXcBn05    => 588,
            Self::LdaC1dCsc       => 18,
            Self::LdaC1dLoos      => 26,
            Self::LdaC2dAmgb      => 15,
            Self::LdaC2dPrm       => 16,
            Self::LdaCChachiyo    => 287,
            Self::LdaCChachiyoMod => 307,
            Self::LdaCGk72        => 578,
            Self::LdaCGombas      => 24,
            Self::LdaCHl          => 4,
            Self::LdaCLp96        => 289,
            Self::LdaCMl1         => 22,
            Self::LdaCPw          => 12,
            Self::LdaCPz          => 9,
            Self::LdaCRc04        => 27,
            Self::LdaCRpa         => 3,
            Self::LdaCVwn         => 7,
            Self::LdaCVwn1        => 28,
            Self::LdaCVwn2        => 29,
            Self::LdaCVwn3        => 30,
            Self::LdaCVwn4        => 31,
            Self::LdaCVwnRpa      => 8,
            Self::LdaCW20         => 317,
            Self::LdaCWigner      => 2,
            Self::LdaKTf          => 50,
            Self::LdaKZlp         => 550,
            Self::LdaX            => 1,
            Self::LdaX2d          => 19,
            Self::LdaXErf         => 546,
            Self::LdaXRel         => 532,
            Self::LdaXSloc        => 692,
            Self::LdaXYukawa      => 641,
            Self::LdaXc1dEhwlrg1  => 536,
            Self::LdaXc1dEhwlrg2  => 537,
            Self::LdaXc1dEhwlrg3  => 538,
            Self::LdaXcTeter93    => 20,
            Self::LdaXcTih        => 599,
            Self::LdaXcZlp        => 43,
        };
        FunctionalId(raw)
    }

    /// True when this functional's kernels include exc_unpol/exc_pol.
    /// False for vxc-only entries (currently only `LdaXcTih`).
    pub fn has_exc(self) -> bool {
        !matches!(self, Self::LdaXcTih)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::FunctionalId;

    #[test]
    fn from_id_routes_lda_x() {
        let id = FunctionalId::from_raw(1).unwrap();
        assert_eq!(LdaFunctional::from_id(id).unwrap(), LdaFunctional::LdaX);
    }

    #[test]
    fn from_id_routes_lda_xc_tih() {
        let id = FunctionalId::from_raw(599).unwrap();
        assert_eq!(LdaFunctional::from_id(id).unwrap(), LdaFunctional::LdaXcTih);
    }

    #[test]
    fn from_id_routes_lda_xc_1d_ehwlrg_variants() {
        assert_eq!(
            LdaFunctional::from_id(FunctionalId::from_raw(536).unwrap()).unwrap(),
            LdaFunctional::LdaXc1dEhwlrg1
        );
        assert_eq!(
            LdaFunctional::from_id(FunctionalId::from_raw(537).unwrap()).unwrap(),
            LdaFunctional::LdaXc1dEhwlrg2
        );
        assert_eq!(
            LdaFunctional::from_id(FunctionalId::from_raw(538).unwrap()).unwrap(),
            LdaFunctional::LdaXc1dEhwlrg3
        );
    }

    #[test]
    fn from_id_rejects_deferred_lda_c_pk09() {
        let id = FunctionalId::from_raw(554).unwrap();
        let err = LdaFunctional::from_id(id).unwrap_err();
        match err {
            LibxcRsError::UnsupportedFunctional { id: e_id, reason } => {
                assert_eq!(e_id.raw(), 554);
                assert!(
                    reason.contains("deferred"),
                    "expected 'deferred' in reason, got: {reason}"
                );
            }
            other => panic!("expected UnsupportedFunctional, got {other:?}"),
        }
    }

    #[test]
    fn from_id_rejects_deferred_lda_xc_ksdt() {
        let id = FunctionalId::from_raw(259).unwrap();
        let err = LdaFunctional::from_id(id).unwrap_err();
        assert!(matches!(err, LibxcRsError::UnsupportedFunctional { .. }));
    }

    #[test]
    fn from_id_rejects_deferred_lda_c_pw_erf() {
        let id = FunctionalId::from_raw(654).unwrap();
        let err = LdaFunctional::from_id(id).unwrap_err();
        assert!(matches!(err, LibxcRsError::UnsupportedFunctional { .. }));
    }

    #[test]
    fn from_id_rejects_deferred_lda_c_pmgb06() {
        let id = FunctionalId::from_raw(590).unwrap();
        let err = LdaFunctional::from_id(id).unwrap_err();
        assert!(matches!(err, LibxcRsError::UnsupportedFunctional { .. }));
    }

    #[test]
    fn from_id_rejects_lda_family_id_not_compiled() {
        // lda_c_xalpha (id 6) is a registry-valid LDA functional, but
        // LdaFunctional doesn't have an enum variant for it. Expect the
        // "no typed enum variant" branch.
        let id = FunctionalId::from_raw(6).unwrap();
        let err = LdaFunctional::from_id(id).unwrap_err();
        match err {
            LibxcRsError::UnsupportedFunctional { id: e_id, reason } => {
                assert_eq!(e_id.raw(), 6);
                assert!(
                    reason.contains("no typed enum variant"),
                    "reason: {reason}"
                );
            }
            other => panic!("expected UnsupportedFunctional, got {other:?}"),
        }
    }

    #[test]
    fn has_exc_false_only_for_lda_xc_tih() {
        assert!(!LdaFunctional::LdaXcTih.has_exc());
        assert!(LdaFunctional::LdaX.has_exc());
        assert!(LdaFunctional::LdaCVwn.has_exc());
        assert!(LdaFunctional::HybLdaXcBn05.has_exc());
    }

    #[test]
    fn to_id_round_trips_through_from_id() {
        let cases = [
            (1, LdaFunctional::LdaX),
            (7, LdaFunctional::LdaCVwn),
            (588, LdaFunctional::HybLdaXcBn05),
            (599, LdaFunctional::LdaXcTih),
            (538, LdaFunctional::LdaXc1dEhwlrg3),
        ];
        for (raw, variant) in cases {
            let id = FunctionalId::from_raw(raw).unwrap();
            assert_eq!(LdaFunctional::from_id(id).unwrap(), variant);
            assert_eq!(variant.to_id().raw(), raw);
        }
    }
}
