//! Runtime Functional handle: owns spin, dimensions, thresholds, ext_params,
//! typed params trait object, and (Plan 05-03) eagerly-constructed auxiliary
//! Functionals.
//!
//! Evaluate methods (`Functional::evaluate_lda` etc.) are added in Plan 05-03;
//! this plan delivers construction (`Functional::new`), config (threshold +
//! ext_param setters/getters), and the `FunctionalParams` plumbing trait.

pub mod config;
pub mod evaluate;
pub mod hybrid;
pub mod lifecycle;
pub mod params;
pub mod params_gga;
pub mod params_lda;
pub mod params_mgga;

pub use hybrid::{classify_hybrid, CamCoefficients, NlcCoefficients};
pub use params::{FunctionalParams, NoParams};

use libxc_core::dims::Dimensions;
use libxc_core::meta::FunctionalMeta;
use libxc_core::model::{Spin, Thresholds};

/// Runtime handle for an XC functional instance. Owns per-instance mutable
/// state (thresholds, ext_params, aux functionals) alongside an immutable
/// reference to static metadata.
///
/// Constructed via [`Functional::new`]. Thread-safe: `Send + Sync`.
pub struct Functional {
    pub(crate) meta: &'static FunctionalMeta,
    pub(crate) spin: Spin,
    pub(crate) dims: Dimensions,
    pub(crate) thresholds: Thresholds,
    /// `None` when `meta.ext_params.is_empty()` — preserves EVAL-04 zero-alloc
    /// for the majority of functionals.
    /// `Some(Box<[f64]>)` initialized from `meta.ext_params[i].default_value`.
    pub(crate) ext_params: Option<Box<[f64]>>,
    /// Per-functional derived params (raw + derived scalars for dispatch downcast).
    pub(crate) params: Box<dyn FunctionalParams>,
    /// Eagerly-recursively-constructed auxiliary functionals.
    /// Empty in Plan 05-02; populated in Plan 05-03.
    #[allow(dead_code)]
    pub(crate) auxiliaries: Vec<Functional>,
    /// Mixing coefficients aligned with `auxiliaries` (copied from `meta.auxiliaries[i].1`).
    /// Empty in Plan 05-02; populated in Plan 05-03.
    #[allow(dead_code)]
    pub(crate) mix_coefficients: Vec<f64>,
}

impl Functional {
    /// Returns the static metadata reference (zero-cost).
    pub fn meta(&self) -> &'static FunctionalMeta {
        self.meta
    }

    /// Returns the spin mode this Functional was constructed for.
    pub fn spin(&self) -> Spin {
        self.spin
    }

    /// Returns the per-family per-spin derivative dimensions.
    pub fn dims(&self) -> &Dimensions {
        &self.dims
    }

    /// Returns the current numerical thresholds (density/zeta/sigma/tau).
    pub fn thresholds(&self) -> &Thresholds {
        &self.thresholds
    }

    /// Borrow the dynamic params trait object (for dispatch).
    pub fn params(&self) -> &dyn FunctionalParams {
        self.params.as_ref()
    }
}

#[cfg(test)]
mod thread_safety_tests {
    use super::Functional;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn functional_is_send_sync() {
        // Compile-time enforcement of D-13: Functional must be Send + Sync.
        assert_send_sync::<Functional>();
    }
}
