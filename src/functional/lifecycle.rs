//! `Functional::new` (constructor) + `Drop` (no-op) + `construct_params`
//! dispatch table mapping `meta.id` to a concrete `Box<dyn FunctionalParams>`.
//!
//! Plan 05-03 layered eager recursive auxiliary construction here: when
//! `meta.auxiliaries` is non-empty, `Functional::new` constructs each aux
//! Functional (depth bound 2 enforced by xtask) and then applies the static
//! `PROPAGATION_RULES` so aux ext_params reflect the parent's defaults.

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::functional::params::{FunctionalParams, NoParams};
use crate::functional::{params_lda, Functional};
use crate::meta::generated_propagation::PROPAGATION_RULES;
use crate::meta::FunctionalMeta;
use crate::model::{Family, FunctionalId, Spin, Thresholds};
use crate::registry::lookup_by_id;

impl Functional {
    /// Construct a `Functional` for the given id and spin mode.
    ///
    /// # Arguments
    /// * `id` — Validated `FunctionalId`. Look up via `FunctionalId::from_raw`
    ///   or `FunctionalId::from_name` first.
    /// * `spin` — `Spin::Unpolarized` or `Spin::Polarized`.
    ///
    /// # Errors
    /// * `UnknownFunctionalId(raw)` if the id is outside the 649-functional set.
    /// * `RemovedFunctionalId { .. }` if the id was removed in libxc 7.0.0
    ///   (only id 104 is truly removed; others are aliases — the registry
    ///   resolves automatically).
    pub fn new(id: FunctionalId, spin: Spin) -> Result<Self, LibxcRsError> {
        let meta: &'static FunctionalMeta = lookup_by_id(id.raw())?;

        let dims = match meta.family {
            Family::Lda => Dimensions::lda(spin),
            Family::Gga => Dimensions::gga(spin),
            Family::Mgga => Dimensions::mgga(spin),
        };

        // ext_params storage per D-06: None when meta.ext_params.is_empty(),
        // Some(Box<[f64]>) initialized from default_value otherwise.
        let ext_params: Option<Box<[f64]>> = if meta.ext_params.is_empty() {
            None
        } else {
            Some(
                meta.ext_params
                    .iter()
                    .map(|spec| spec.default_value)
                    .collect::<Vec<f64>>()
                    .into_boxed_slice(),
            )
        };

        // Per-functional params construction. Plan 05-02 scope:
        // - LdaX (id 1) gets concrete LdaXParams (the only LDA functional whose
        //   dispatch arm currently consumes a runtime ext_param via downcast).
        // - All other ids get NoParams (the dispatch arms continue to use
        //   hardcoded libxc defaults).
        let params: Box<dyn FunctionalParams> = construct_params(meta.id, ext_params.as_deref())?;

        // Eager recursive aux construction (Plan 05-03 D-15). Depth ≤ 2
        // (enforced at xtask snapshot time, see Plan 05-01 D-17). On the
        // current empty-metadata snapshot every functional has
        // `meta.auxiliaries: &[]` so this loop is a no-op; once xtask
        // populates real auxiliaries, this performs the recursion.
        let mut auxiliaries: Vec<Functional> = Vec::with_capacity(meta.auxiliaries.len());
        let mut mix_coefficients: Vec<f64> = Vec::with_capacity(meta.auxiliaries.len());
        for &(aux_id, weight) in meta.auxiliaries {
            let aux = Functional::new(aux_id, spin).map_err(|e| {
                LibxcRsError::AuxiliaryInitFailed {
                    parent_id: meta.id,
                    aux_id,
                    source: Box::new(e),
                }
            })?;
            auxiliaries.push(aux);
            mix_coefficients.push(weight);
        }

        let mut out = Functional {
            meta,
            spin,
            dims,
            thresholds: Thresholds::default(),
            ext_params,
            params,
            auxiliaries,
            mix_coefficients,
        };

        // Apply propagation rules: for each rule whose parent_id matches
        // self.meta.id, copy the parent's ext_param value (read by index
        // for O(1) access; xtask validates indices at snapshot time) into
        // the named aux slot's ext_param.
        out.propagate_to_aux()?;

        Ok(out)
    }

    /// Apply `PROPAGATION_RULES` for this functional id to its auxiliary
    /// functionals. Called from `Functional::new` and from any ext_param
    /// setter that mutates parent state (`config.rs::set_ext_params`).
    ///
    /// Returns `PropagationConflict` if any rule references an out-of-range
    /// `parent_param_index`, an `aux_slot >= auxiliaries.len()`, or an
    /// `aux_param_name` not present on the targeted aux. Real
    /// `PROPAGATION_RULES` (xtask-validated) never trigger these branches;
    /// they exist as defense-in-depth against snapshot drift.
    pub(crate) fn propagate_to_aux(&mut self) -> Result<(), LibxcRsError> {
        let id = self.meta.id;
        let ext_snapshot: Option<Vec<f64>> =
            self.ext_params.as_deref().map(|s| s.to_vec());
        for rule in PROPAGATION_RULES.iter().filter(|r| r.parent_id == id) {
            let ext = ext_snapshot.as_deref().ok_or(
                LibxcRsError::PropagationConflict {
                    id,
                    parent_name: rule.parent_param_name,
                    aux_slot: rule.aux_slot,
                    aux_name: rule.aux_param_name,
                },
            )?;
            let parent_value = ext
                .get(rule.parent_param_index as usize)
                .copied()
                .ok_or(LibxcRsError::PropagationConflict {
                    id,
                    parent_name: rule.parent_param_name,
                    aux_slot: rule.aux_slot,
                    aux_name: rule.aux_param_name,
                })?;
            let aux = self
                .auxiliaries
                .get_mut(rule.aux_slot as usize)
                .ok_or(LibxcRsError::PropagationConflict {
                    id,
                    parent_name: rule.parent_param_name,
                    aux_slot: rule.aux_slot,
                    aux_name: rule.aux_param_name,
                })?;
            aux.set_ext_param(rule.aux_param_name, parent_value)
                .map_err(|_| LibxcRsError::PropagationConflict {
                    id,
                    parent_name: rule.parent_param_name,
                    aux_slot: rule.aux_slot,
                    aux_name: rule.aux_param_name,
                })?;
        }
        Ok(())
    }
}

impl Drop for Functional {
    /// No-op per D-15. All fields (Box, Vec, &'static) auto-drop. Implemented
    /// explicitly so downstream readers do not wonder whether there is FFI or
    /// other resource cleanup that needs to happen.
    fn drop(&mut self) {
        // Intentionally empty.
    }
}

/// Dispatch table from `meta.id` to a concrete `FunctionalParams` impl.
///
/// Plan 05-02 scope: only `LdaX` (id 1) gets a concrete impl beyond the
/// `NoParams` blanket. All other ids fall through to `NoParams` — the
/// dispatch arms in `src/eval/{dispatch,gga_dispatch,mgga_dispatch}.rs`
/// continue to use hardcoded libxc defaults at the call site, so this
/// is bit-for-bit equivalent to the pre-Plan-05-02 behavior.
///
/// Follow-up plans will extend this match to cover the 36 ext-param-bearing
/// LDA functionals + ~25 ext-param-bearing GGA + ~15 ext-param-bearing MGGA.
pub(crate) fn construct_params(
    id: FunctionalId,
    _defaults: Option<&[f64]>,
) -> Result<Box<dyn FunctionalParams>, LibxcRsError> {
    match id.raw() {
        1 => {
            // XC_LDA_X: alpha (Slater scaling). Concrete params for downcast.
            let p = params_lda::LdaXParams::from_defaults();
            Ok(Box::new(p))
        }
        _ => {
            // All other functionals: NoParams. Dispatch arms ignore the
            // trait-object params and use hardcoded libxc defaults.
            Ok(Box::new(NoParams))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Spin;

    #[test]
    fn new_lda_x_unpolarized_succeeds() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert_eq!(f.meta.id.raw(), 1);
        assert_eq!(f.spin, Spin::Unpolarized);
        assert_eq!(f.thresholds.density, 1e-15);
    }

    #[test]
    fn new_lda_x_polarized_succeeds() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Polarized).unwrap();
        assert_eq!(f.spin, Spin::Polarized);
        assert_eq!(f.dims.vrho, 2);
        assert_eq!(f.dims.v2rho2, 3);
    }

    #[test]
    fn new_unknown_id_returns_err() {
        // FunctionalId::from_raw rejects unknown ids; bypass by constructing
        // raw via the pub(crate) tuple ctor — but we cannot from outside, so
        // we test via from_raw.
        let result = FunctionalId::from_raw(9999);
        assert!(result.is_err());
    }

    #[test]
    fn new_lda_family_dims_match_lda_helper() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        let expected = Dimensions::lda(Spin::Unpolarized);
        assert_eq!(f.dims.rho, expected.rho);
        assert_eq!(f.dims.zk, expected.zk);
        assert_eq!(f.dims.vrho, expected.vrho);
    }

    #[test]
    fn new_gga_family_dims_match_gga_helper() {
        // pbe (id 130, gga_x_pbe) — take any GGA id from the registry.
        // Use lookup_by_name for robustness.
        let id =
            FunctionalId::from_name("gga_x_pbe").unwrap_or_else(|_| FunctionalId::from_raw(101).unwrap());
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert_eq!(f.meta.family, Family::Gga);
        let expected = Dimensions::gga(Spin::Unpolarized);
        assert_eq!(f.dims.sigma, expected.sigma);
        assert_eq!(f.dims.vsigma, expected.vsigma);
    }

    #[test]
    fn new_mgga_family_dims_match_mgga_helper() {
        // tpss (id 202, mgga_x_tpss) — typical MGGA.
        let id = FunctionalId::from_name("mgga_x_tpss")
            .unwrap_or_else(|_| FunctionalId::from_raw(202).unwrap());
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert_eq!(f.meta.family, Family::Mgga);
        let expected = Dimensions::mgga(Spin::Unpolarized);
        assert_eq!(f.dims.tau, expected.tau);
        assert_eq!(f.dims.vtau, expected.vtau);
    }

    #[test]
    fn new_zero_ext_params_functional_has_none() {
        // lda_x has 1 ext_param when meta is populated (Plan 05-01 deferred
        // populating ext_params; current generated.rs has all empty).
        // So for *any* id we currently get None.
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // If meta.ext_params is empty, ext_params is None.
        if f.meta.ext_params.is_empty() {
            assert!(f.ext_params.is_none());
        } else {
            assert!(f.ext_params.is_some());
            let arr = f.ext_params.as_deref().unwrap();
            assert_eq!(arr.len(), f.meta.ext_params.len());
            for (i, spec) in f.meta.ext_params.iter().enumerate() {
                assert_eq!(arr[i], spec.default_value);
            }
        }
    }

    #[test]
    fn drop_is_no_op_no_panic() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        drop(f); // Should not panic.
    }

    #[test]
    fn lda_x_construct_params_yields_lda_x_params_concrete() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // The trait object should downcast to LdaXParams for id=1.
        let downcast = f.params.as_any().downcast_ref::<params_lda::LdaXParams>();
        assert!(downcast.is_some(), "LdaX construct_params should yield LdaXParams");
        assert_eq!(downcast.unwrap().alpha, 1.0);
    }

    #[test]
    fn non_lda_x_construct_params_yields_no_params() {
        let id =
            FunctionalId::from_name("gga_x_pbe").unwrap_or_else(|_| FunctionalId::from_raw(101).unwrap());
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // GGA functionals fall through to NoParams in Plan 05-02.
        let downcast = f.params.as_any().downcast_ref::<NoParams>();
        assert!(downcast.is_some(), "Non-LDA-X functionals should yield NoParams in Plan 05-02");
    }

    // ── Plan 05-03: aux recursion + propagation invariants ────────────

    /// Test 2 (aux depth bounded): for every id in the registry, recursive
    /// aux depth observed at construction time is <= 2 (xtask invariant
    /// D-17). Currently meta.auxiliaries is empty for all 649 ids (Plan
    /// 05-01 deferred), so observed depth is 0 — the test still verifies
    /// the construction loop terminates without panicking.
    #[test]
    fn aux_depth_bounded_for_all_649_ids() {
        use crate::registry::all_functional_ids;
        for id in all_functional_ids() {
            // Construct and immediately drop. Recursion bound enforced
            // at xtask snapshot time; we just confirm runtime construction
            // does not loop or panic.
            let f = Functional::new(id, Spin::Unpolarized);
            // Some ids may construct successfully even when their kernel
            // is deferred — Functional::new itself never fails on
            // deferred ids, only evaluate_* does (Pitfall 7).
            if let Ok(func) = f {
                // Walk auxiliary tree, asserting depth <= 2.
                fn walk(f: &Functional, depth: u8) {
                    assert!(
                        depth <= 2,
                        "aux depth {depth} exceeds bound 2 for id {}",
                        f.meta.id
                    );
                    for aux in f.auxiliary_functionals() {
                        walk(aux, depth + 1);
                    }
                }
                walk(&func, 0);
            }
        }
    }

    /// FUNC-06: Drop must not panic for representative hybrid ids.
    /// After Plan 05-04 metadata population, hybrid functionals carry
    /// non-empty `auxiliaries` so this test now exercises the recursive
    /// Drop path through the eagerly-constructed sub-Functionals.
    #[test]
    fn drop_hybrids_ok() {
        // 10 representative hybrid ids: B3LYP family, CAM-B3LYP, wB97X,
        // M06, HSE03, PBE0, B2PLYP, X3LYP, LC-wPBE, mgga_c_b94_hyb. Some
        // of these may not be in the registry (b94_hyb is mgga family);
        // we use lookup_by_name and skip any that don't resolve.
        let candidate_names = [
            "hyb_gga_xc_b3lyp",
            "hyb_gga_xc_cam_b3lyp",
            "hyb_gga_xc_wb97x",
            "hyb_mgga_xc_m06",
            "hyb_gga_xc_hse03",
            "hyb_gga_xc_pbeh",
            "hyb_gga_xc_b2plyp",
            "hyb_mgga_xc_x1b95",
            "hyb_gga_xc_lc_wpbe",
            "mgga_c_b94_hyb",
        ];
        let mut count = 0usize;
        let mut nonempty_aux = 0usize;
        for name in candidate_names {
            if let Ok(id) = FunctionalId::from_name(name) {
                let f = Functional::new(id, Spin::Unpolarized);
                if let Ok(func) = f {
                    if !func.auxiliary_functionals().is_empty() {
                        nonempty_aux += 1;
                    }
                    drop(func);
                    count += 1;
                }
            }
        }
        // Sanity: at least some of the candidate names resolve in the
        // current registry.
        assert!(count > 0, "no candidate hybrid ids resolved");
        // Plan 05-04 closure: at least one canonical hybrid (e.g. B3LYP)
        // must carry non-empty aux after metadata population.
        assert!(
            nonempty_aux > 0,
            "expected at least one hybrid candidate with non-empty aux after Plan 05-04"
        );
    }

    /// Plan 05-04 closure: B3LYP's metadata-driven aux count is 4
    /// (LDA_X, GGA_X_B88, LDA_C_VWN_RPA, GGA_C_LYP). After
    /// `cargo xtask generate-metadata` populates `meta.auxiliaries`
    /// from a live `xc_func_init` snapshot, both the static slice and
    /// the recursively-constructed `Vec<Functional>` carry 4 entries.
    #[test]
    fn b3lyp_aux_count_is_4() {
        let id = FunctionalId::from_name("hyb_gga_xc_b3lyp").unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert_eq!(
            f.meta.auxiliaries.len(),
            4,
            "B3LYP must have 4 auxiliaries"
        );
        assert_eq!(
            f.auxiliary_functionals().len(),
            4,
            "Recursive aux construction must populate 4 sub-Functionals"
        );
    }

    /// Propagation map runtime application is a no-op when
    /// PROPAGATION_RULES is empty (current state). This test guards the
    /// loop's defensive error paths.
    #[test]
    fn empty_propagation_rules_runs_clean() {
        // Construct several ids; if propagate_to_aux had any unguarded
        // panic path it would surface here.
        for raw in [1u16, 101, 130, 202, 287] {
            if let Ok(id) = FunctionalId::from_raw(raw) {
                let _ = Functional::new(id, Spin::Unpolarized).unwrap();
            }
        }
    }
}
