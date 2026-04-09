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

/// Internal struct for LDA field offsets within the MGGA-ordered scratch buffer.
#[allow(dead_code)]
struct LdaFieldOffsets {
    zk_off: usize,
    zk_len: usize,
    vrho_off: usize,
    vrho_len: usize,
    v2rho2_off: usize,
    v2rho2_len: usize,
    v3rho3_off: usize,
    v3rho3_len: usize,
    v4rho4_off: usize,
    v4rho4_len: usize,
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

    /// Compute the element offset of a field in the MGGA-ordered scratch buffer.
    ///
    /// The scratch layout follows the field order in `Dimensions::total_output_components()`:
    /// Order 0: zk
    /// Order 1: vrho, vsigma, vlapl, vtau
    /// Order 2: v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2, v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2
    /// Order 3: v3rho3, ...
    /// Order 4: v4rho4, ...
    ///
    /// Returns (offset_in_elements, field_len_in_elements) for the requested field.
    fn lda_field_offsets(&self) -> LdaFieldOffsets {
        let d = &self.dims;
        let np = self.np;

        // Order 0
        let zk_off = 0usize;
        let zk_len = d.zk as usize * np;

        // Order 1: vrho is first
        let o1_start = zk_len;
        let vrho_off = o1_start;
        let vrho_len = d.vrho as usize * np;
        let o1_end = o1_start
            + (d.vrho as usize + d.vsigma as usize + d.vlapl as usize + d.vtau as usize) * np;

        // Order 2: v2rho2 is first in order 2
        let o2_start = o1_end;
        let v2rho2_off = o2_start;
        let v2rho2_len = d.v2rho2 as usize * np;
        let o2_end = o2_start
            + (d.v2rho2 as usize
                + d.v2rhosigma as usize
                + d.v2rholapl as usize
                + d.v2rhotau as usize
                + d.v2sigma2 as usize
                + d.v2sigmalapl as usize
                + d.v2sigmatau as usize
                + d.v2lapl2 as usize
                + d.v2lapltau as usize
                + d.v2tau2 as usize)
                * np;

        // Order 3: v3rho3 is first in order 3
        let o3_start = o2_end;
        let v3rho3_off = o3_start;
        let v3rho3_len = d.v3rho3 as usize * np;
        let o3_end = o3_start
            + (d.v3rho3 as usize
                + d.v3rho2sigma as usize
                + d.v3rho2lapl as usize
                + d.v3rho2tau as usize
                + d.v3rhosigma2 as usize
                + d.v3rhosigmalapl as usize
                + d.v3rhosigmatau as usize
                + d.v3rholapl2 as usize
                + d.v3rholapltau as usize
                + d.v3rhotau2 as usize
                + d.v3sigma3 as usize
                + d.v3sigma2lapl as usize
                + d.v3sigma2tau as usize
                + d.v3sigmalapl2 as usize
                + d.v3sigmalapltau as usize
                + d.v3sigmatau2 as usize
                + d.v3lapl3 as usize
                + d.v3lapl2tau as usize
                + d.v3lapltau2 as usize
                + d.v3tau3 as usize)
                * np;

        // Order 4: v4rho4 is first in order 4
        let v4rho4_off = o3_end;
        let v4rho4_len = d.v4rho4 as usize * np;

        LdaFieldOffsets {
            zk_off,
            zk_len,
            vrho_off,
            vrho_len,
            v2rho2_off,
            v2rho2_len,
            v3rho3_off,
            v3rho3_len,
            v4rho4_off,
            v4rho4_len,
        }
    }

    /// Get mutable scratch slices for LDA derivative fields.
    ///
    /// Returns non-overlapping slices via `split_at_mut` into the contiguous
    /// scratch buffer. The slices correspond to the LDA fields (zk, vrho,
    /// v2rho2, v3rho3, v4rho4) at their correct offsets within the MGGA-ordered
    /// layout.
    pub fn lda_scratch_mut(&mut self) -> LdaScratch<'_> {
        let offsets = self.lda_field_offsets();

        // Use split_at_mut to create non-overlapping mutable slices.
        // We split progressively, keeping track of the "rest" slice.
        // Since fields are at known offsets, we skip over non-LDA fields.

        // zk is at offset 0
        let (zk_and_rest, after_zk) = self.scratch.split_at_mut(offsets.zk_len);
        let zk = &mut zk_and_rest[..offsets.zk_len];

        // vrho starts at vrho_off, which is right after zk for MGGA (zk is order 0, vrho is first in order 1)
        // The gap between end of zk and start of vrho is 0 (vrho immediately follows zk)
        let vrho_local_off = offsets.vrho_off - offsets.zk_len;
        let (_, vrho_start) = after_zk.split_at_mut(vrho_local_off);
        let (vrho, after_vrho) = vrho_start.split_at_mut(offsets.vrho_len);

        // v2rho2 starts at v2rho2_off
        let v2rho2_local_off =
            offsets.v2rho2_off - offsets.vrho_off - offsets.vrho_len;
        let (_, v2rho2_start) = after_vrho.split_at_mut(v2rho2_local_off);
        let (v2rho2, after_v2rho2) = v2rho2_start.split_at_mut(offsets.v2rho2_len);

        // v3rho3
        let v3rho3_local_off =
            offsets.v3rho3_off - offsets.v2rho2_off - offsets.v2rho2_len;
        let (_, v3rho3_start) = after_v2rho2.split_at_mut(v3rho3_local_off);
        let (v3rho3, after_v3rho3) = v3rho3_start.split_at_mut(offsets.v3rho3_len);

        // v4rho4
        let v4rho4_local_off =
            offsets.v4rho4_off - offsets.v3rho3_off - offsets.v3rho3_len;
        let (_, v4rho4_start) = after_v3rho3.split_at_mut(v4rho4_local_off);
        let (v4rho4, _) = v4rho4_start.split_at_mut(offsets.v4rho4_len);

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
