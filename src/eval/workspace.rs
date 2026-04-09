//! Pre-allocated scratch buffer management for mixed functional evaluation.
//!
//! Mixed/hybrid functionals evaluate multiple auxiliary functionals and combine
//! their results with weights. `EvaluationWorkspace` provides a single contiguous
//! scratch allocation sized for the MGGA superset (D-12), enabling zero-allocation
//! evaluation loops.

use crate::dims::Dimensions;
use crate::model::Spin;

/// Scratch slices for LDA derivative fields.
///
/// All slices are non-overlapping views into the workspace's contiguous buffer,
/// created via `split_at_mut` chains.
pub struct LdaScratch<'a> {
    pub zk: &'a mut [f64],
    pub vrho: &'a mut [f64],
    pub v2rho2: &'a mut [f64],
    pub v3rho3: &'a mut [f64],
    pub v4rho4: &'a mut [f64],
}

/// Placeholder for GGA scratch slices (Phase 4).
pub struct GgaScratch<'a> {
    _marker: std::marker::PhantomData<&'a mut [f64]>,
}

/// Placeholder for MGGA scratch slices (Phase 4).
pub struct MggaScratch<'a> {
    _marker: std::marker::PhantomData<&'a mut [f64]>,
}

/// Pre-allocated scratch buffer for mixed functional evaluation.
///
/// Allocates a single contiguous `Vec<f64>` sized for the MGGA superset
/// (767 components for polarized, per D-12). This ensures any family's
/// auxiliary functional can write into the scratch without reallocation.
///
/// # Usage
/// ```ignore
/// let mut ws = EvaluationWorkspace::new(np, spin);
/// for aux in auxiliaries {
///     ws.zero_scratch();
///     // evaluate aux into ws.lda_scratch_mut()
///     // accumulate into final output
/// }
/// ```
pub struct EvaluationWorkspace {
    scratch: Vec<f64>,
    np: usize,
    spin: Spin,
    dims: Dimensions,
}

impl EvaluationWorkspace {
    /// Create a new workspace with scratch sized for MGGA superset.
    ///
    /// The scratch buffer has `dims.total_output_components() * np` elements,
    /// all initialized to 0.0.
    pub fn new(np: usize, spin: Spin) -> Self {
        let dims = Dimensions::mgga(spin);
        let total = dims.total_output_components() * np;
        Self {
            scratch: vec![0.0; total],
            np,
            spin,
            dims,
        }
    }

    /// Zero all scratch buffer elements.
    ///
    /// Must be called before each auxiliary evaluation to prevent
    /// cross-contamination (T-03-07, T-03-08 mitigation).
    pub fn zero_scratch(&mut self) {
        self.scratch.fill(0.0);
    }

    /// Number of grid points this workspace was sized for.
    pub fn np(&self) -> usize {
        self.np
    }

    /// Spin mode this workspace was sized for.
    pub fn spin(&self) -> Spin {
        self.spin
    }

    /// Get mutable scratch slices for LDA derivative fields.
    ///
    /// Returns non-overlapping slices via `split_at_mut` chains into the
    /// contiguous scratch buffer. The slices are at the beginning of the
    /// buffer, covering only the LDA-relevant fields (zk, vrho, v2rho2,
    /// v3rho3, v4rho4).
    pub fn lda_scratch_mut(&mut self) -> LdaScratch<'_> {
        let zk_len = self.dims.zk as usize * self.np;
        let vrho_len = self.dims.vrho as usize * self.np;
        let v2rho2_len = self.dims.v2rho2 as usize * self.np;
        let v3rho3_len = self.dims.v3rho3 as usize * self.np;
        let v4rho4_len = self.dims.v4rho4 as usize * self.np;

        let (zk, rest) = self.scratch.split_at_mut(zk_len);
        let (vrho, rest) = rest.split_at_mut(vrho_len);
        let (v2rho2, rest) = rest.split_at_mut(v2rho2_len);
        let (v3rho3, rest) = rest.split_at_mut(v3rho3_len);
        let (v4rho4, _rest) = rest.split_at_mut(v4rho4_len);

        LdaScratch {
            zk,
            vrho,
            v2rho2,
            v3rho3,
            v4rho4,
        }
    }

    /// Get mutable scratch slices for GGA derivative fields.
    ///
    /// Phase 4 will implement this. Currently panics.
    pub fn gga_scratch_mut(&mut self) -> GgaScratch<'_> {
        // Phase 4 will implement GGA scratch accessor
        todo!("GGA scratch accessor not yet implemented -- Phase 4")
    }

    /// Get mutable scratch slices for MGGA derivative fields.
    ///
    /// Phase 4 will implement this. Currently panics.
    pub fn mgga_scratch_mut(&mut self) -> MggaScratch<'_> {
        // Phase 4 will implement MGGA scratch accessor
        todo!("MGGA scratch accessor not yet implemented -- Phase 4")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dims::Dimensions;
    use crate::model::Spin;

    #[test]
    fn new_unpolarized_allocates_correct_size() {
        let np = 100;
        let ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let expected = Dimensions::mgga(Spin::Unpolarized).total_output_components() * np;
        assert_eq!(ws.scratch.len(), expected);
        assert_eq!(ws.np(), np);
        assert_eq!(ws.spin(), Spin::Unpolarized);
    }

    #[test]
    fn new_polarized_allocates_correct_size() {
        let np = 100;
        let ws = EvaluationWorkspace::new(np, Spin::Polarized);
        let expected = Dimensions::mgga(Spin::Polarized).total_output_components() * np;
        // 767 * 100 = 76700
        assert_eq!(expected, 76700);
        assert_eq!(ws.scratch.len(), expected);
    }

    #[test]
    fn zero_scratch_zeroes_all_elements() {
        let np = 10;
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        // Write non-zero values
        for val in ws.scratch.iter_mut() {
            *val = 42.0;
        }
        ws.zero_scratch();
        for (i, &val) in ws.scratch.iter().enumerate() {
            assert_eq!(val, 0.0, "scratch[{i}] should be 0.0 after zero_scratch");
        }
    }

    #[test]
    fn lda_scratch_unpolarized_correct_lengths() {
        let np = 10;
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let scratch = ws.lda_scratch_mut();
        // Unpolarized LDA: all dims are 1
        assert_eq!(scratch.zk.len(), np);      // zk: 1 * np
        assert_eq!(scratch.vrho.len(), np);     // vrho: 1 * np
        assert_eq!(scratch.v2rho2.len(), np);   // v2rho2: 1 * np
        assert_eq!(scratch.v3rho3.len(), np);   // v3rho3: 1 * np
        assert_eq!(scratch.v4rho4.len(), np);   // v4rho4: 1 * np
    }

    #[test]
    fn lda_scratch_polarized_correct_lengths() {
        let np = 10;
        let mut ws = EvaluationWorkspace::new(np, Spin::Polarized);
        let scratch = ws.lda_scratch_mut();
        // Polarized LDA: vrho=2, v2rho2=3, v3rho3=4, v4rho4=5
        assert_eq!(scratch.zk.len(), np);           // zk: 1 * np
        assert_eq!(scratch.vrho.len(), 2 * np);     // vrho: 2 * np
        assert_eq!(scratch.v2rho2.len(), 3 * np);   // v2rho2: 3 * np
        assert_eq!(scratch.v3rho3.len(), 4 * np);   // v3rho3: 4 * np
        assert_eq!(scratch.v4rho4.len(), 5 * np);   // v4rho4: 5 * np
    }

    #[test]
    fn scratch_reuse_write_zero_verify() {
        let np = 5;
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        // Write to scratch via LDA accessor
        {
            let scratch = ws.lda_scratch_mut();
            for val in scratch.zk.iter_mut() {
                *val = 1.0;
            }
            for val in scratch.vrho.iter_mut() {
                *val = 2.0;
            }
        }
        // Zero and verify
        ws.zero_scratch();
        {
            let scratch = ws.lda_scratch_mut();
            for &val in scratch.zk.iter() {
                assert_eq!(val, 0.0);
            }
            for &val in scratch.vrho.iter() {
                assert_eq!(val, 0.0);
            }
        }
    }

    #[test]
    fn lda_scratch_after_zero_all_zeros() {
        let np = 20;
        let mut ws = EvaluationWorkspace::new(np, Spin::Polarized);
        ws.zero_scratch();
        let scratch = ws.lda_scratch_mut();
        for &v in scratch.zk.iter() { assert_eq!(v, 0.0); }
        for &v in scratch.vrho.iter() { assert_eq!(v, 0.0); }
        for &v in scratch.v2rho2.iter() { assert_eq!(v, 0.0); }
        for &v in scratch.v3rho3.iter() { assert_eq!(v, 0.0); }
        for &v in scratch.v4rho4.iter() { assert_eq!(v, 0.0); }
    }
}
