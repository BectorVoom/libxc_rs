//! `Functional::new` (constructor) + `Drop` (no-op) + `construct_params`
//! dispatch table mapping `meta.id` to a concrete `Box<dyn FunctionalParams>`.

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::functional::params::{FunctionalParams, NoParams};
use crate::functional::{params_lda, Functional};
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

        Ok(Functional {
            meta,
            spin,
            dims,
            thresholds: Thresholds::default(),
            ext_params,
            params,
            auxiliaries: Vec::new(),
            mix_coefficients: Vec::new(),
        })
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
}
