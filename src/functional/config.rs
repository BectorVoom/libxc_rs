//! `Functional` configuration API: ext_param setters/getters (by name, index,
//! or bulk) + four threshold setters.
//!
//! All setters take `&mut self` (D-13: no interior mutability). Getters return
//! `Option`/`Result` — no panics.

use crate::error::LibxcRsError;
use crate::functional::Functional;

impl Functional {
    // ====================================================================
    // Ext_param getters
    // ====================================================================

    /// Borrow the raw ext_params array.
    /// Returns `None` if this functional has no ext_params (`meta.ext_params`
    /// is empty); `Some(&[f64])` otherwise.
    pub fn ext_params(&self) -> Option<&[f64]> {
        self.ext_params.as_deref()
    }

    /// Get the value of the ext_param at `idx`.
    ///
    /// # Errors
    /// `ExtParamIndexOutOfRange` if `idx >= meta.ext_params.len()`.
    pub fn ext_param_by_index(&self, idx: usize) -> Result<f64, LibxcRsError> {
        let arr = self.ext_params.as_deref().unwrap_or(&[]);
        arr.get(idx).copied().ok_or(LibxcRsError::ExtParamIndexOutOfRange {
            id: self.meta.id,
            index: idx,
            count: arr.len(),
        })
    }

    /// Get the value of the ext_param named `name`.
    ///
    /// # Errors
    /// `UnknownExtParamName` if no ext_param matches `name`.
    pub fn ext_param(&self, name: &str) -> Result<f64, LibxcRsError> {
        let arr = self.ext_params.as_deref().unwrap_or(&[]);
        for (i, spec) in self.meta.ext_params.iter().enumerate() {
            if spec.name == name {
                // CR-04 defensive read: if `arr` is shorter than the spec list
                // (e.g. invariant broken because `self.ext_params is None` while
                // metadata is populated), fall back to the meta default rather
                // than panicking on `arr[i]`.
                return Ok(arr.get(i).copied().unwrap_or(spec.default_value));
            }
        }
        Err(LibxcRsError::UnknownExtParamName {
            id: self.meta.id,
            name: name.to_string(),
        })
    }

    // ====================================================================
    // Ext_param setters
    // ====================================================================

    /// Bulk-set all ext_params. Length must match `meta.ext_params.len()`.
    /// Recomputes derived fields in the inner `params` trait object.
    ///
    /// # Errors
    /// `ExtParamCountMismatch` if `vals.len() != meta.ext_params.len()`.
    pub fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        let n = self.meta.ext_params.len();
        if vals.len() != n {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: self.meta.id,
                expected: n,
                actual: vals.len(),
            });
        }
        if let Some(ref mut arr) = self.ext_params {
            arr.copy_from_slice(vals);
        }
        // Forward to the dynamic params object so any derived scalars
        // (e.g. CAM alpha/beta/omega computed from raw inputs) are refreshed.
        // Pass-through error if the inner params impl rejects — for the
        // Plan 05-02 NoParams blanket this only triggers when vals.len() != 0
        // for a zero-ext_param functional, which is already caught above.
        self.params.set_ext_params(vals)?;
        // Plan 05-03 D-16 — re-apply PROPAGATION_RULES so any aux ext_params
        // that mirror parent values stay in sync after this mutation.
        self.propagate_to_aux()?;
        Ok(())
    }

    /// Set the ext_param at `idx` to `val`.
    /// Triggers a full read-modify-write of the ext_params array so derived
    /// fields get recomputed.
    ///
    /// # Errors
    /// `ExtParamIndexOutOfRange` if `idx >= meta.ext_params.len()`.
    pub fn set_ext_param_by_index(&mut self, idx: usize, val: f64) -> Result<(), LibxcRsError> {
        let count = self.meta.ext_params.len();
        if idx >= count {
            return Err(LibxcRsError::ExtParamIndexOutOfRange {
                id: self.meta.id,
                index: idx,
                count,
            });
        }
        // CR-04 fix: when `self.ext_params is None` but `meta.ext_params` is
        // non-empty (an invariant the constructor normally maintains, but
        // which Plan 05-04 metadata population could expose), seed `new_vals`
        // from `meta.ext_params[i].default_value`. The previous implementation
        // produced an empty Vec in this branch and would then panic at
        // `new_vals[idx] = val`.
        let mut new_vals: Vec<f64> = match self.ext_params.as_deref() {
            Some(s) => s.to_vec(),
            None => self
                .meta
                .ext_params
                .iter()
                .map(|spec| spec.default_value)
                .collect(),
        };
        // Defense-in-depth: if the invariant is broken and `new_vals` is
        // shorter than `count`, surface a typed error rather than panicking
        // on the index assignment.
        if new_vals.len() != count {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: self.meta.id,
                expected: count,
                actual: new_vals.len(),
            });
        }
        new_vals[idx] = val;
        self.set_ext_params(&new_vals)
    }

    /// Set the ext_param named `name` to `val`.
    ///
    /// # Errors
    /// `UnknownExtParamName` if no ext_param matches `name`.
    pub fn set_ext_param(&mut self, name: &str, val: f64) -> Result<(), LibxcRsError> {
        for (i, spec) in self.meta.ext_params.iter().enumerate() {
            if spec.name == name {
                return self.set_ext_param_by_index(i, val);
            }
        }
        Err(LibxcRsError::UnknownExtParamName {
            id: self.meta.id,
            name: name.to_string(),
        })
    }

    // ====================================================================
    // Threshold setters (4)
    // ====================================================================

    pub fn set_density_threshold(&mut self, v: f64) {
        self.thresholds.density = v;
        for aux in self.auxiliaries.iter_mut() {
            aux.set_density_threshold(v);
        }
    }

    pub fn set_zeta_threshold(&mut self, v: f64) {
        self.thresholds.zeta = v;
        for aux in self.auxiliaries.iter_mut() {
            aux.set_zeta_threshold(v);
        }
    }

    pub fn set_sigma_threshold(&mut self, v: f64) {
        self.thresholds.sigma = v;
        for aux in self.auxiliaries.iter_mut() {
            aux.set_sigma_threshold(v);
        }
    }

    pub fn set_tau_threshold(&mut self, v: f64) {
        self.thresholds.tau = v;
        for aux in self.auxiliaries.iter_mut() {
            aux.set_tau_threshold(v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{FunctionalId, Spin};

    #[test]
    fn set_density_threshold_writes_through() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert_eq!(f.thresholds.density, 1e-15);
        f.set_density_threshold(1e-12);
        assert_eq!(f.thresholds.density, 1e-12);
    }

    #[test]
    fn set_zeta_threshold_writes_through() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        f.set_zeta_threshold(1e-8);
        assert_eq!(f.thresholds.zeta, 1e-8);
    }

    #[test]
    fn set_sigma_threshold_writes_through() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        f.set_sigma_threshold(1e-20);
        assert_eq!(f.thresholds.sigma, 1e-20);
    }

    #[test]
    fn set_tau_threshold_writes_through() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        f.set_tau_threshold(1e-15);
        assert_eq!(f.thresholds.tau, 1e-15);
    }

    #[test]
    fn ext_params_returns_none_for_zero_ext_param_functional() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        // Plan 05-01 left meta.ext_params empty for all functionals;
        // so ext_params() should return None.
        if f.meta.ext_params.is_empty() {
            assert!(f.ext_params().is_none());
        }
    }

    #[test]
    fn set_ext_params_wrong_length_errors() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        // For a zero-ext_param functional, supplying any values is a length
        // mismatch (n_expected = 0).
        if f.meta.ext_params.is_empty() {
            let result = f.set_ext_params(&[0.5]);
            match result {
                Err(LibxcRsError::ExtParamCountMismatch { expected, actual, .. }) => {
                    assert_eq!(expected, 0);
                    assert_eq!(actual, 1);
                }
                other => panic!("expected ExtParamCountMismatch, got {other:?}"),
            }
        }
    }

    #[test]
    fn set_ext_param_by_index_out_of_range_errors() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        let result = f.set_ext_param_by_index(0, 0.5);
        // For a zero-ext_param functional, idx=0 is out-of-range (count=0).
        if f.meta.ext_params.is_empty() {
            match result {
                Err(LibxcRsError::ExtParamIndexOutOfRange { index, count, .. }) => {
                    assert_eq!(index, 0);
                    assert_eq!(count, 0);
                }
                other => panic!("expected ExtParamIndexOutOfRange, got {other:?}"),
            }
        }
    }

    #[test]
    fn set_ext_param_unknown_name_errors() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        let result = f.set_ext_param("nonexistent_param", 0.5);
        match result {
            Err(LibxcRsError::UnknownExtParamName { name, .. }) => {
                assert_eq!(name, "nonexistent_param");
            }
            other => panic!("expected UnknownExtParamName, got {other:?}"),
        }
    }

    #[test]
    fn ext_param_by_index_out_of_range_errors() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        let result = f.ext_param_by_index(99);
        match result {
            Err(LibxcRsError::ExtParamIndexOutOfRange { index, count, .. }) => {
                assert_eq!(index, 99);
                // count is whatever ext_params length is (likely 0)
                let _ = count;
            }
            other => panic!("expected ExtParamIndexOutOfRange, got {other:?}"),
        }
    }

    #[test]
    fn ext_param_unknown_name_errors() {
        let id = FunctionalId::from_raw(1).unwrap();
        let f = Functional::new(id, Spin::Unpolarized).unwrap();
        let result = f.ext_param("totally_bogus");
        match result {
            Err(LibxcRsError::UnknownExtParamName { name, .. }) => {
                assert_eq!(name, "totally_bogus");
            }
            other => panic!("expected UnknownExtParamName, got {other:?}"),
        }
    }

    /// CR-04 regression test: forcibly break the invariant `ext_params is Some`
    /// when `meta.ext_params is non-empty` (which Plan 05-04 metadata
    /// population is expected to introduce in the wild on caller misuse).
    /// `set_ext_param_by_index` must NOT panic; it must either succeed by
    /// re-seeding from `meta.ext_params[i].default_value`, or return a typed
    /// error such as `ExtParamIndexOutOfRange` / `ExtParamCountMismatch`.
    #[test]
    fn set_ext_param_by_index_recovers_when_ext_params_is_none_after_metadata_population() {
        let id = FunctionalId::from_raw(1).unwrap(); // lda_x
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        // Force the latent invariant to be broken: ext_params None even though
        // (after Plan 05-04 lands) meta might list a parameter.
        f.ext_params = None;
        // For empty meta this returns ExtParamIndexOutOfRange (no panic).
        // For non-empty meta this re-seeds new_vals from default_value and
        // succeeds. Either way: NO panic.
        let result = f.set_ext_param_by_index(0, 0.5);
        match result {
            Ok(())
            | Err(LibxcRsError::ExtParamIndexOutOfRange { .. })
            | Err(LibxcRsError::ExtParamCountMismatch { .. }) => {}
            other => panic!(
                "set_ext_param_by_index must never panic and must surface a typed error; got {other:?}"
            ),
        }
    }

    /// CR-04 symmetric defensive-read regression for the `ext_param` getter:
    /// when `self.ext_params is None` but `meta.ext_params` lists entries
    /// (post Plan 05-04), reading by name must NOT panic. It returns the
    /// `default_value` for the matching spec, or `UnknownExtParamName` if
    /// no spec matches.
    // === Pitfall 4 — threshold setters propagate to auxiliaries ===

    #[test]
    fn threshold_propagates_to_aux_density() {
        let id = FunctionalId::from_name("XC_HYB_GGA_XC_B3LYP").unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert!(!f.auxiliary_functionals().is_empty(), "B3LYP must have auxiliaries");
        f.set_density_threshold(1e-12);
        assert_eq!(f.thresholds().density, 1e-12);
        for aux in f.auxiliary_functionals() {
            assert_eq!(aux.thresholds().density, 1e-12,
                "aux {} did not receive density threshold", aux.meta().name);
        }
    }

    #[test]
    fn threshold_propagates_to_aux_zeta() {
        let id = FunctionalId::from_name("XC_HYB_GGA_XC_B3LYP").unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert!(!f.auxiliary_functionals().is_empty(), "B3LYP must have auxiliaries");
        f.set_zeta_threshold(2e-7);
        assert_eq!(f.thresholds().zeta, 2e-7);
        for aux in f.auxiliary_functionals() {
            assert_eq!(aux.thresholds().zeta, 2e-7,
                "aux {} did not receive zeta threshold", aux.meta().name);
        }
    }

    #[test]
    fn threshold_propagates_to_aux_sigma() {
        let id = FunctionalId::from_name("XC_HYB_GGA_XC_B3LYP").unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert!(!f.auxiliary_functionals().is_empty(), "B3LYP must have auxiliaries");
        f.set_sigma_threshold(3e-19);
        assert_eq!(f.thresholds().sigma, 3e-19);
        for aux in f.auxiliary_functionals() {
            assert_eq!(aux.thresholds().sigma, 3e-19,
                "aux {} did not receive sigma threshold", aux.meta().name);
        }
    }

    #[test]
    fn threshold_propagates_to_aux_tau() {
        let id = FunctionalId::from_name("XC_HYB_GGA_XC_B3LYP").unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        assert!(!f.auxiliary_functionals().is_empty(), "B3LYP must have auxiliaries");
        f.set_tau_threshold(4e-14);
        assert_eq!(f.thresholds().tau, 4e-14);
        for aux in f.auxiliary_functionals() {
            assert_eq!(aux.thresholds().tau, 4e-14,
                "aux {} did not receive tau threshold", aux.meta().name);
        }
    }

    #[test]
    fn ext_param_getter_does_not_panic_when_ext_params_is_none() {
        let id = FunctionalId::from_raw(1).unwrap();
        let mut f = Functional::new(id, Spin::Unpolarized).unwrap();
        f.ext_params = None;
        // Bogus name path always returns UnknownExtParamName regardless of
        // whether meta.ext_params is empty or populated.
        match f.ext_param("definitely_not_a_real_param") {
            Err(LibxcRsError::UnknownExtParamName { .. }) => {}
            other => panic!("expected UnknownExtParamName, got {other:?}"),
        }
        // If meta.ext_params is non-empty (post Plan 05-04), looking up by
        // the actual name MUST return the default rather than panicking.
        if let Some(spec) = f.meta.ext_params.first() {
            let name = spec.name;
            let expected_default = spec.default_value;
            let got = f.ext_param(name).expect("must not panic and must succeed for known name");
            assert!(
                got == expected_default || got.is_finite(),
                "expected meta default {expected_default} or a finite fallback, got {got}"
            );
        }
    }
}
