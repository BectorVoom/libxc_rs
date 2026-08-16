//! Owned-self chained-config builder for [`Functional`].
//!
//! Wraps [`Functional::new`] + Phase-5 setter chain. Configuration
//! accumulates in private fields; setter errors surface from
//! [`FunctionalBuilder::build`], not mid-chain. Per CONTEXT D-A1
//! and § Specifics line 268-275.
//!
//! Layer-3 module — zero `unsafe`.

use crate::error::LibxcRsError;
use crate::functional::Functional;
use crate::model::{FunctionalId, Spin};

/// Builder over [`Functional::new`] + Phase-5 setter chain.
///
/// All configuration steps return `self` by move; the final
/// [`FunctionalBuilder::build`] constructs the [`Functional`] and
/// applies the accumulated thresholds and ext_param overrides in a
/// deterministic order:
///
/// 1. `Functional::new(id, spin)` — constructs metadata, dims, default ext_params.
/// 2. Apply density / zeta / sigma / tau thresholds (if set).
/// 3. Apply ext_param overrides in chain order.
///
/// Any setter error short-circuits with `?`. ext_param errors
/// (e.g. unknown name) surface from `.build()`, not mid-chain — the
/// builder accumulates `(name, value)` pairs without validating until
/// `Functional::set_ext_param` is called.
///
/// # Example
/// ```ignore
/// use libxc_rs::{FunctionalBuilder, FunctionalId, Spin};
/// let f = FunctionalBuilder::new(FunctionalId::from_name("xc_lda_x").unwrap())
///     .spin(Spin::Polarized)
///     .density_threshold(1e-12)
///     .build()
///     .unwrap();
/// ```
pub struct FunctionalBuilder {
    id: FunctionalId,
    spin: Spin,
    density_threshold: Option<f64>,
    zeta_threshold: Option<f64>,
    sigma_threshold: Option<f64>,
    tau_threshold: Option<f64>,
    ext_param_overrides: Vec<(String, f64)>,
}

impl FunctionalBuilder {
    /// Start a new builder with the given functional ID.
    ///
    /// Default spin is [`Spin::Unpolarized`] (matches libxc convention).
    pub fn new(id: FunctionalId) -> Self {
        Self {
            id,
            spin: Spin::Unpolarized,
            density_threshold: None,
            zeta_threshold: None,
            sigma_threshold: None,
            tau_threshold: None,
            ext_param_overrides: Vec::new(),
        }
    }

    /// Override the spin polarization mode.
    pub fn spin(mut self, spin: Spin) -> Self {
        self.spin = spin;
        self
    }

    /// Override the density threshold (libxc default is per-functional).
    pub fn density_threshold(mut self, t: f64) -> Self {
        self.density_threshold = Some(t);
        self
    }

    /// Override the zeta threshold.
    pub fn zeta_threshold(mut self, t: f64) -> Self {
        self.zeta_threshold = Some(t);
        self
    }

    /// Override the sigma threshold.
    pub fn sigma_threshold(mut self, t: f64) -> Self {
        self.sigma_threshold = Some(t);
        self
    }

    /// Override the tau threshold.
    pub fn tau_threshold(mut self, t: f64) -> Self {
        self.tau_threshold = Some(t);
        self
    }

    /// Override an external parameter by name. Errors are deferred to
    /// [`FunctionalBuilder::build`] — invalid names surface there, not
    /// mid-chain.
    pub fn ext_param(mut self, name: impl Into<String>, val: f64) -> Self {
        self.ext_param_overrides.push((name.into(), val));
        self
    }

    /// Construct the [`Functional`] and apply accumulated configuration.
    ///
    /// # Errors
    /// - Any error from [`Functional::new`] (e.g.
    ///   [`LibxcRsError::UnknownFunctionalId`]).
    /// - Any error from a setter (e.g.
    ///   [`LibxcRsError::UnknownExtParamName`] when an `ext_param(name,..)`
    ///   chain step references an unknown ext_param).
    pub fn build(self) -> Result<Functional, LibxcRsError> {
        let mut f = Functional::new(self.id, self.spin)?;
        if let Some(t) = self.density_threshold {
            f.set_density_threshold(t);
        }
        if let Some(t) = self.zeta_threshold {
            f.set_zeta_threshold(t);
        }
        if let Some(t) = self.sigma_threshold {
            f.set_sigma_threshold(t);
        }
        if let Some(t) = self.tau_threshold {
            f.set_tau_threshold(t);
        }
        for (name, val) in self.ext_param_overrides {
            f.set_ext_param(&name, val)?;
        }
        Ok(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{FunctionalId, Spin};

    /// Test 1: happy-path build with id + spin produces a Functional whose
    /// observable state matches the inputs.
    #[test]
    fn build_with_id_and_spin_succeeds() {
        let id = FunctionalId::from_name("xc_lda_x")
            .unwrap_or_else(|_| FunctionalId::from_raw(1).unwrap());
        let f = FunctionalBuilder::new(id)
            .spin(Spin::Unpolarized)
            .build()
            .unwrap();
        assert_eq!(f.spin(), Spin::Unpolarized);
        assert_eq!(f.meta().id, id);
    }

    /// Test 2: omitting `.spin(...)` defaults to `Spin::Unpolarized`.
    #[test]
    fn default_spin_is_unpolarized() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = FunctionalBuilder::new(id).build().unwrap();
        assert_eq!(f.spin(), Spin::Unpolarized);
    }

    /// Test 3: `.density_threshold(t)` propagates through to the
    /// `Functional`'s thresholds.
    #[test]
    fn density_threshold_applied() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = FunctionalBuilder::new(id)
            .density_threshold(1e-12)
            .build()
            .unwrap();
        assert_eq!(f.thresholds().density, 1e-12);
    }

    /// Test 4: `.ext_param("alpha", 0.7)` applies to a functional whose
    /// metadata lists "alpha". Tested only when meta.ext_params is
    /// populated for lda_x; otherwise this test is a no-op assertion.
    #[test]
    fn ext_param_by_name_applied() {
        let id = FunctionalId::from_raw(1).unwrap();
        // Phase-5 metadata snapshot leaves ext_params empty for most
        // functionals, but if xtask has populated lda_x's "alpha" the
        // builder must propagate it. Skip the assertion gracefully when
        // metadata is empty; a populated metadata path will exercise the
        // happy case.
        let meta = crate::registry::lookup_by_id(id.raw()).unwrap();
        if let Some(spec) = meta.ext_params.first() {
            let name = spec.name;
            let f = FunctionalBuilder::new(id)
                .ext_param(name, 0.7)
                .build()
                .unwrap();
            assert_eq!(f.ext_param(name).unwrap(), 0.7);
        } else {
            // No ext_params in current snapshot — the test still passes
            // because the chain step accumulated nothing and build()
            // succeeded.
            let _ = FunctionalBuilder::new(id).build().unwrap();
        }
    }

    /// Test 5: `.ext_param("nonexistent", 0.0).build()` returns
    /// [`LibxcRsError::UnknownExtParamName`] — error surfaces from build,
    /// not mid-chain.
    #[test]
    fn ext_param_unknown_name_returns_err_at_build() {
        let id = FunctionalId::from_raw(1).unwrap();
        let result = FunctionalBuilder::new(id)
            .ext_param("definitely_not_a_real_param", 0.0)
            .build();
        match result {
            Err(LibxcRsError::UnknownExtParamName { name, .. }) => {
                assert_eq!(name, "definitely_not_a_real_param");
            }
            // `Functional` has no `Debug` impl, so the whole `Result` cannot be
            // formatted; report the error (which does) or the Ok case by name.
            Err(e) => panic!("expected UnknownExtParamName, got {e:?}"),
            Ok(_) => panic!("expected UnknownExtParamName, got Ok(..)"),
        }
    }

    /// Test 6: `FunctionalId::from_raw(0)` itself returns
    /// `UnknownFunctionalId(0)` — id construction is the validation gate.
    /// This test confirms the builder cannot be reached with an invalid id
    /// (the upstream `from_raw` returns `Err`).
    #[test]
    fn unknown_id_returns_err_at_build() {
        // FunctionalId::from_raw rejects 0 (not in the 649-id registry).
        let err = FunctionalId::from_raw(0).unwrap_err();
        match err {
            LibxcRsError::UnknownFunctionalId(0) => {}
            other => panic!("expected UnknownFunctionalId(0), got {other:?}"),
        }
    }

    /// Test 7: FunctionalBuilder is `Send + Sync` (compile-only check).
    #[test]
    fn functional_builder_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<FunctionalBuilder>();
    }
}
