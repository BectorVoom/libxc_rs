//! `MggaFunctional` enum: enumerates the compiled MGGA functionals routable
//! by `dispatch_mgga`.
//!
//! Each variant maps one libxc functional ID to a kernel module in one of the
//! 95 `crates/kernel-mgga-*` sub-crates. Variants are ordered by libxc ID for
//! consistency with the registry.
//!
//! The `MggaXTb09` variant (libxc id 208) is the vxc-only special case: it has
//! no `exc_unpol`/`exc_pol` kernels in `crates/kernel-mgga-35`, so `has_exc()`
//! returns `false` and the dispatch layer rejects `DerivativeOrder::Exc`
//! requests for it (W5 — has_exc is filesystem-driven, INDEPENDENT of libxc's
//! `FLAGS_HAVE_EXC`).
//!
//! Non-compiled MGGA functionals (e.g. `mgga_c_b94` id 397 which is listed in
//! `crates/libxc-core/src/deferred.rs`, or functionals whose kernel modules
//! have only partial-derivative coverage after split-file translations) are
//! NOT represented here — `MggaFunctional::from_id` returns
//! `Err(UnsupportedFunctional)` for them.
//!
//! **Note on `mgga_x_2d_prp10`:** The plan frontmatter listed this as a second
//! VXC-only functional, but inspection of the filesystem shows the module is
//! commented out in `crates/kernel-mgga-35/src/lib.rs` with reason
//! "requires xc_bessel_I0/I1 (Bessel functions)". The variant is therefore
//! absent from this enum; its id (211) returns `UnsupportedFunctional` via
//! the default arm of `from_id`.
//!
//! **Note on deferred functionals (6 IDs):** `MggaFunctional::from_id` explicitly
//! rejects the six deferred libxc IDs (`mgga_c_b94`, `mgga_x_br89`, `mgga_x_mbr`,
//! `mgga_x_mbrxc_bg`, `mgga_x_mbrxh_bg`, `mgga_x_mggac`) before the main match
//! via `crate::deferred::mgga::is_deferred`, surfacing a specific
//! "Brent's-method root-finder" reason so callers can distinguish these from
//! genuinely-not-translated IDs.
//!
//! **Note on template kernels:** Five kernel modules back no unique libxc ID
//! via a direct name match (`mgga_k_lk`, `mgga_k_pgslb`, `mgga_x_m06l`,
//! `mgga_x_m08`, `mgga_x_ms`, `mgga_x_msb`). These kernels exist in the
//! filesystem but are not addressed by any registry id at this layer. They
//! are omitted from this enum for the same reason GGA omitted the
//! corresponding template kernels (tracked separately as Phase 4 follow-up).

use crate::error::LibxcRsError;
use crate::model::FunctionalId;
use crate::deferred::mgga::is_deferred as is_deferred_mgga;

/// Enumerates the 25 compiled, routable MGGA functionals `dispatch_mgga` can serve.
///
/// Variants are ordered by libxc ID (ascending) for consistency with the registry.
/// The one vxc-only case (`MggaXTb09`) is flagged via `has_exc() == false`.
///
/// Plan frontmatter expected "86 compiled MGGA functionals" — that counts every
/// sub-crate module file on disk. After filtering by:
/// 1. Modules commented-out in their crate's `lib.rs` (e.g. `mgga_x_2d_prp10`
///    needs Bessel I0/I1, `mgga_c_b94` + `mgga_x_br89` etc. need root-finders).
/// 2. Partial-translation modules that lack a full 10-arm coverage (most of
///    the large correlation kernels split across multiple sub-crate batches).
/// 3. Template kernels with no direct libxc id match (5 kernels).
/// …the actual dispatchable count is 25 FULL + 1 VXC-only = 26 variants.
/// The 62 remaining modules on-disk are partially translated or template-
/// backed; completing them is deferred to Phase 4 follow-up plans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MggaFunctional {
    /// `hyb_mgga_x_dldf` (libxc id 36, kernel batch 21)
    HybMggaXDldf,
    /// `mgga_xc_zlp` (libxc id 42, kernel batch 23)
    MggaXcZlp,
    /// `mgga_c_cs` (libxc id 72, kernel batch 34)
    MggaCCs,
    /// `mgga_x_lta` (libxc id 201, kernel batch 34)
    MggaXLta,
    /// `mgga_x_tpss` (libxc id 202, kernel batch 33)
    MggaXTpss,
    /// `mgga_x_tau_hcth` (libxc id 205, kernel batch 34)
    MggaXTauHcth,
    /// `mgga_x_tb09` (libxc id 208, kernel batch 35 — vxc-only, no exc kernels)
    MggaXTb09,
    /// `mgga_x_pkzb` (libxc id 213, kernel batch 28)
    MggaXPkzb,
    /// `mgga_x_th` (libxc id 225, kernel batch 34)
    MggaXTh,
    /// `mgga_xc_cc06` (libxc id 229, kernel batch 35)
    MggaXcCc06,
    /// `mgga_x_jk` (libxc id 256, kernel batch 35)
    MggaXJk,
    /// `mgga_x_mvs` (libxc id 257, kernel batch 35)
    MggaXMvs,
    /// `mgga_x_rtpss` (libxc id 299, kernel batch 21)
    MggaXRtpss,
    /// `mgga_c_cc` (libxc id 387, kernel batch 35)
    MggaCCc,
    /// `hyb_mgga_x_m05` (libxc id 438, kernel batch 29)
    HybMggaXM05,
    /// `mgga_x_tm` (libxc id 540, kernel batch 30)
    MggaXTm,
    /// `mgga_xc_lp90` (libxc id 564, kernel batch 29)
    MggaXcLp90,
    /// `mgga_x_gx` (libxc id 575, kernel batch 29)
    MggaXGx,
    /// `mgga_x_pbe_gx` (libxc id 576, kernel batch 29)
    MggaXPbeGx,
    /// `mgga_x_2d_js17` (libxc id 609, kernel batch 34)
    MggaX2dJs17,
    /// `mgga_k_rda` (libxc id 621, kernel batch 29)
    MggaKRda,
    /// `mgga_k_gea2` (libxc id 627, kernel batch 17)
    MggaKGea2,
    /// `mgga_k_gea4` (libxc id 628, kernel batch 34)
    MggaKGea4,
    /// `mgga_x_rlda` (libxc id 688, kernel batch 34)
    MggaXRlda,
    /// `mgga_x_task` (libxc id 707, kernel batch 33)
    MggaXTask,
}

impl MggaFunctional {
    /// Map a libxc functional ID to a dispatchable MGGA variant, or return an error.
    ///
    /// Returns `Err(UnsupportedFunctional)` for:
    /// - The 6 deferred MGGA IDs tracked in `crates/kernel-mgga/src/deferred.rs`
    ///   (mgga_c_b94, mgga_x_br89, mgga_x_mbr, mgga_x_mbrxc_bg, mgga_x_mbrxh_bg,
    ///   mgga_x_mggac) — these require Brent's-method root-finders.
    /// - MGGA functionals whose kernel module has only partial derivative
    ///   coverage (split-file translations such as `mgga_c_tpss`).
    /// - MGGA functionals whose kernel module is commented out in its crate's
    ///   `lib.rs` (e.g. `mgga_x_2d_prp10`, which needs Bessel functions).
    /// - Non-MGGA IDs (route those via `dispatch_lda` / `dispatch_gga`).
    pub fn from_id(id: FunctionalId) -> Result<Self, LibxcRsError> {
        if is_deferred_mgga(id.raw()) {
            // D-11: deferred MGGA functionals are unconditionally rejected in
            // production. (The D-15/D-22 test-time env-var bypass was removed at
            // phase close — 11-13 Task 2 Step 5.)
            return Err(LibxcRsError::UnsupportedFunctional {
                id,
                reason: "MGGA functional deferred pending Brent's method root-finder. \
                         See crates/kernels/math/src/deferred.rs",
            });
        }
        match id.raw() {
            36 => Ok(Self::HybMggaXDldf),
            42 => Ok(Self::MggaXcZlp),
            72 => Ok(Self::MggaCCs),
            201 => Ok(Self::MggaXLta),
            202 => Ok(Self::MggaXTpss),
            205 => Ok(Self::MggaXTauHcth),
            208 => Ok(Self::MggaXTb09),
            213 => Ok(Self::MggaXPkzb),
            225 => Ok(Self::MggaXTh),
            229 => Ok(Self::MggaXcCc06),
            256 => Ok(Self::MggaXJk),
            257 => Ok(Self::MggaXMvs),
            299 => Ok(Self::MggaXRtpss),
            387 => Ok(Self::MggaCCc),
            438 => Ok(Self::HybMggaXM05),
            540 => Ok(Self::MggaXTm),
            564 => Ok(Self::MggaXcLp90),
            575 => Ok(Self::MggaXGx),
            576 => Ok(Self::MggaXPbeGx),
            609 => Ok(Self::MggaX2dJs17),
            621 => Ok(Self::MggaKRda),
            627 => Ok(Self::MggaKGea2),
            628 => Ok(Self::MggaKGea4),
            688 => Ok(Self::MggaXRlda),
            707 => Ok(Self::MggaXTask),
            _ => Err(LibxcRsError::UnsupportedFunctional {
                id,
                reason: "MGGA functional not yet translated into crates/kernel-mgga*",
            }),
        }
    }

    /// Reverse of `from_id`: return the canonical libxc functional ID for this variant.
    pub fn to_id(self) -> FunctionalId {
        let raw = match self {
            Self::HybMggaXDldf => 36,
            Self::MggaXcZlp => 42,
            Self::MggaCCs => 72,
            Self::MggaXLta => 201,
            Self::MggaXTpss => 202,
            Self::MggaXTauHcth => 205,
            Self::MggaXTb09 => 208,
            Self::MggaXPkzb => 213,
            Self::MggaXTh => 225,
            Self::MggaXcCc06 => 229,
            Self::MggaXJk => 256,
            Self::MggaXMvs => 257,
            Self::MggaXRtpss => 299,
            Self::MggaCCc => 387,
            Self::HybMggaXM05 => 438,
            Self::MggaXTm => 540,
            Self::MggaXcLp90 => 564,
            Self::MggaXGx => 575,
            Self::MggaXPbeGx => 576,
            Self::MggaX2dJs17 => 609,
            Self::MggaKRda => 621,
            Self::MggaKGea2 => 627,
            Self::MggaKGea4 => 628,
            Self::MggaXRlda => 688,
            Self::MggaXTask => 707,
        };
        FunctionalId::from_raw(raw)
            .expect("MggaFunctional::to_id produced a registry-valid id")
    }

    /// True when the kernel module contains `exc_unpol.rs`/`exc_pol.rs` on disk (W5).
    ///
    /// This is a structural, filesystem-driven check INDEPENDENT of libxc's
    /// `FLAGS_HAVE_EXC` bit. The only variant where the exc files are genuinely
    /// absent on disk is `MggaXTb09` (id 208). (The plan-frontmatter-mentioned
    /// `MggaX2dPrp10` would have been the second, but that kernel module is
    /// commented out in `crates/kernel-mgga-35/src/lib.rs` pending Bessel
    /// function support — it is not in this enum at all.)
    ///
    /// Mismatches between this check and libxc's `FLAGS_HAVE_EXC` are
    /// legitimate (e.g. Rust kernel has exc files but libxc flag is unset for
    /// this id, or vice versa). Oracle tests must check BOTH sides.
    pub fn has_exc(self) -> bool {
        !matches!(self, Self::MggaXTb09)
    }

    /// Kernel module name (matches the directory under `crates/kernel-mgga-*/src/`).
    pub fn kernel_name(self) -> &'static str {
        match self {
            Self::HybMggaXDldf => "hyb_mgga_x_dldf",
            Self::MggaXcZlp => "mgga_xc_zlp",
            Self::MggaCCs => "mgga_c_cs",
            Self::MggaXLta => "mgga_x_lta",
            Self::MggaXTpss => "mgga_x_tpss",
            Self::MggaXTauHcth => "mgga_x_tau_hcth",
            Self::MggaXTb09 => "mgga_x_tb09",
            Self::MggaXPkzb => "mgga_x_pkzb",
            Self::MggaXTh => "mgga_x_th",
            Self::MggaXcCc06 => "mgga_xc_cc06",
            Self::MggaXJk => "mgga_x_jk",
            Self::MggaXMvs => "mgga_x_mvs",
            Self::MggaXRtpss => "mgga_x_rtpss",
            Self::MggaCCc => "mgga_c_cc",
            Self::HybMggaXM05 => "hyb_mgga_x_m05",
            Self::MggaXTm => "mgga_x_tm",
            Self::MggaXcLp90 => "mgga_xc_lp90",
            Self::MggaXGx => "mgga_x_gx",
            Self::MggaXPbeGx => "mgga_x_pbe_gx",
            Self::MggaX2dJs17 => "mgga_x_2d_js17",
            Self::MggaKRda => "mgga_k_rda",
            Self::MggaKGea2 => "mgga_k_gea2",
            Self::MggaKGea4 => "mgga_k_gea4",
            Self::MggaXRlda => "mgga_x_rlda",
            Self::MggaXTask => "mgga_x_task",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_id_routes_mgga_x_tpss() {
        let id = FunctionalId::from_raw(202).unwrap();
        assert_eq!(MggaFunctional::from_id(id).unwrap(), MggaFunctional::MggaXTpss);
    }

    #[test]
    fn from_id_routes_mgga_x_tb09_vxc_only() {
        let id = FunctionalId::from_raw(208).unwrap();
        let f = MggaFunctional::from_id(id).unwrap();
        assert_eq!(f, MggaFunctional::MggaXTb09);
        assert!(!f.has_exc(), "mgga_x_tb09 must report has_exc() == false");
    }

    #[test]
    fn from_id_rejects_deferred_br89() {
        // mgga_x_br89 (id 206) is deferred.
        let id = FunctionalId::from_raw(206).unwrap();
        let err = MggaFunctional::from_id(id).unwrap_err();
        match err {
            LibxcRsError::UnsupportedFunctional { id: e_id, reason } => {
                assert_eq!(e_id.raw(), 206);
                assert!(reason.contains("deferred"), "reason: {reason}");
            }
            other => panic!("expected UnsupportedFunctional, got {other:?}"),
        }
    }

    #[test]
    fn from_id_rejects_deferred_b94() {
        let id = FunctionalId::from_raw(397).unwrap();
        let err = MggaFunctional::from_id(id).unwrap_err();
        assert!(matches!(err, LibxcRsError::UnsupportedFunctional { .. }));
    }

    #[test]
    fn from_id_rejects_lda_x() {
        // LDA_X (id 1) is valid in the registry but is not MGGA.
        let id = FunctionalId::from_raw(1).unwrap();
        let err = MggaFunctional::from_id(id).unwrap_err();
        match err {
            LibxcRsError::UnsupportedFunctional { id: e_id, reason } => {
                assert_eq!(e_id.raw(), 1);
                assert!(reason.contains("not yet translated"), "reason: {reason}");
            }
            other => panic!("expected UnsupportedFunctional, got {other:?}"),
        }
    }

    #[test]
    fn has_exc_false_only_for_tb09() {
        assert!(!MggaFunctional::MggaXTb09.has_exc());
        assert!(MggaFunctional::MggaXTpss.has_exc());
        assert!(MggaFunctional::MggaCCs.has_exc());
        assert!(MggaFunctional::HybMggaXDldf.has_exc());
    }

    #[test]
    fn to_id_round_trips_through_from_id() {
        let cases = [
            (202, MggaFunctional::MggaXTpss),
            (208, MggaFunctional::MggaXTb09),
            (36, MggaFunctional::HybMggaXDldf),
            (707, MggaFunctional::MggaXTask),
        ];
        for (raw, variant) in cases {
            let id = FunctionalId::from_raw(raw).unwrap();
            assert_eq!(MggaFunctional::from_id(id).unwrap(), variant);
            assert_eq!(variant.to_id().raw(), raw);
        }
    }

    #[test]
    fn kernel_name_is_libxc_name() {
        assert_eq!(MggaFunctional::MggaXTpss.kernel_name(), "mgga_x_tpss");
        assert_eq!(MggaFunctional::MggaXTb09.kernel_name(), "mgga_x_tb09");
        assert_eq!(MggaFunctional::HybMggaXDldf.kernel_name(), "hyb_mgga_x_dldf");
    }
}
