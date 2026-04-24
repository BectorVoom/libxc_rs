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

/// Scratch slices for GGA derivative fields.
///
/// All slices are non-overlapping views into the workspace's contiguous buffer,
/// created via `split_at_mut` chains. 15 fields covering zk + 2 order-1 +
/// 3 order-2 + 4 order-3 + 5 order-4.
pub struct GgaScratch<'a> {
    pub zk: &'a mut [f64],
    pub vrho: &'a mut [f64],
    pub vsigma: &'a mut [f64],
    pub v2rho2: &'a mut [f64],
    pub v2rhosigma: &'a mut [f64],
    pub v2sigma2: &'a mut [f64],
    pub v3rho3: &'a mut [f64],
    pub v3rho2sigma: &'a mut [f64],
    pub v3rhosigma2: &'a mut [f64],
    pub v3sigma3: &'a mut [f64],
    pub v4rho4: &'a mut [f64],
    pub v4rho3sigma: &'a mut [f64],
    pub v4rho2sigma2: &'a mut [f64],
    pub v4rhosigma3: &'a mut [f64],
    pub v4sigma4: &'a mut [f64],
}

/// Scratch slices for MGGA derivative fields.
///
/// All slices are non-overlapping views into the workspace's contiguous buffer,
/// created via `split_at_mut` chains. 70 fields covering the full MGGA
/// superset (zk + 4 order-1 + 10 order-2 + 20 order-3 + 35 order-4).
pub struct MggaScratch<'a> {
    pub zk: &'a mut [f64],
    // Order 1
    pub vrho: &'a mut [f64],
    pub vsigma: &'a mut [f64],
    pub vlapl: &'a mut [f64],
    pub vtau: &'a mut [f64],
    // Order 2
    pub v2rho2: &'a mut [f64],
    pub v2rhosigma: &'a mut [f64],
    pub v2rholapl: &'a mut [f64],
    pub v2rhotau: &'a mut [f64],
    pub v2sigma2: &'a mut [f64],
    pub v2sigmalapl: &'a mut [f64],
    pub v2sigmatau: &'a mut [f64],
    pub v2lapl2: &'a mut [f64],
    pub v2lapltau: &'a mut [f64],
    pub v2tau2: &'a mut [f64],
    // Order 3
    pub v3rho3: &'a mut [f64],
    pub v3rho2sigma: &'a mut [f64],
    pub v3rho2lapl: &'a mut [f64],
    pub v3rho2tau: &'a mut [f64],
    pub v3rhosigma2: &'a mut [f64],
    pub v3rhosigmalapl: &'a mut [f64],
    pub v3rhosigmatau: &'a mut [f64],
    pub v3rholapl2: &'a mut [f64],
    pub v3rholapltau: &'a mut [f64],
    pub v3rhotau2: &'a mut [f64],
    pub v3sigma3: &'a mut [f64],
    pub v3sigma2lapl: &'a mut [f64],
    pub v3sigma2tau: &'a mut [f64],
    pub v3sigmalapl2: &'a mut [f64],
    pub v3sigmalapltau: &'a mut [f64],
    pub v3sigmatau2: &'a mut [f64],
    pub v3lapl3: &'a mut [f64],
    pub v3lapl2tau: &'a mut [f64],
    pub v3lapltau2: &'a mut [f64],
    pub v3tau3: &'a mut [f64],
    // Order 4
    pub v4rho4: &'a mut [f64],
    pub v4rho3sigma: &'a mut [f64],
    pub v4rho3lapl: &'a mut [f64],
    pub v4rho3tau: &'a mut [f64],
    pub v4rho2sigma2: &'a mut [f64],
    pub v4rho2sigmalapl: &'a mut [f64],
    pub v4rho2sigmatau: &'a mut [f64],
    pub v4rho2lapl2: &'a mut [f64],
    pub v4rho2lapltau: &'a mut [f64],
    pub v4rho2tau2: &'a mut [f64],
    pub v4rhosigma3: &'a mut [f64],
    pub v4rhosigma2lapl: &'a mut [f64],
    pub v4rhosigma2tau: &'a mut [f64],
    pub v4rhosigmalapl2: &'a mut [f64],
    pub v4rhosigmalapltau: &'a mut [f64],
    pub v4rhosigmatau2: &'a mut [f64],
    pub v4rholapl3: &'a mut [f64],
    pub v4rholapl2tau: &'a mut [f64],
    pub v4rholapltau2: &'a mut [f64],
    pub v4rhotau3: &'a mut [f64],
    pub v4sigma4: &'a mut [f64],
    pub v4sigma3lapl: &'a mut [f64],
    pub v4sigma3tau: &'a mut [f64],
    pub v4sigma2lapl2: &'a mut [f64],
    pub v4sigma2lapltau: &'a mut [f64],
    pub v4sigma2tau2: &'a mut [f64],
    pub v4sigmalapl3: &'a mut [f64],
    pub v4sigmalapl2tau: &'a mut [f64],
    pub v4sigmalapltau2: &'a mut [f64],
    pub v4sigmatau3: &'a mut [f64],
    pub v4lapl4: &'a mut [f64],
    pub v4lapl3tau: &'a mut [f64],
    pub v4lapl2tau2: &'a mut [f64],
    pub v4lapltau3: &'a mut [f64],
    pub v4tau4: &'a mut [f64],
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
    /// Returns 15 non-overlapping mutable slices into the workspace's
    /// MGGA-superset contiguous buffer. Slices for fields outside GGA
    /// (vlapl/vtau/v2*lapl/v2*tau/etc.) are skipped — the buffer covers
    /// them but `GgaScratch` doesn't expose them.
    ///
    /// The MGGA-ordered scratch layout follows
    /// `Dimensions::total_output_components()`'s field ordering exactly.
    pub fn gga_scratch_mut(&mut self) -> GgaScratch<'_> {
        let d = &self.dims;
        let np = self.np;
        let mgga_d = Dimensions::mgga(self.spin);

        // Helper: pop a slice of `count_per_pt * np` from the cursor.
        // Each call advances the cursor and returns the carved &mut [f64].
        let buf = self.scratch.as_mut_slice();
        let mut cursor = buf;

        // === Order 0 ===
        let (zk, rest) = cursor.split_at_mut(d.zk as usize * np);
        cursor = rest;

        // === Order 1: vrho, vsigma, vlapl, vtau (MGGA layout) ===
        let (vrho, rest) = cursor.split_at_mut(d.vrho as usize * np);
        cursor = rest;
        let (vsigma, rest) = cursor.split_at_mut(d.vsigma as usize * np);
        cursor = rest;
        // Skip vlapl + vtau (MGGA-only fields, zero-length for GGA d.)
        let (_, rest) = cursor.split_at_mut(mgga_d.vlapl as usize * np + mgga_d.vtau as usize * np);
        cursor = rest;

        // === Order 2 (10 MGGA fields, 3 of which GGA exposes) ===
        let (v2rho2, rest) = cursor.split_at_mut(d.v2rho2 as usize * np);
        cursor = rest;
        let (v2rhosigma, rest) = cursor.split_at_mut(d.v2rhosigma as usize * np);
        cursor = rest;
        // Skip v2rholapl + v2rhotau
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v2rholapl as usize * np + mgga_d.v2rhotau as usize * np,
        );
        cursor = rest;
        let (v2sigma2, rest) = cursor.split_at_mut(d.v2sigma2 as usize * np);
        cursor = rest;
        // Skip v2sigmalapl + v2sigmatau + v2lapl2 + v2lapltau + v2tau2
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v2sigmalapl as usize * np
                + mgga_d.v2sigmatau as usize * np
                + mgga_d.v2lapl2 as usize * np
                + mgga_d.v2lapltau as usize * np
                + mgga_d.v2tau2 as usize * np,
        );
        cursor = rest;

        // === Order 3 (20 MGGA fields, 4 of which GGA exposes) ===
        let (v3rho3, rest) = cursor.split_at_mut(d.v3rho3 as usize * np);
        cursor = rest;
        let (v3rho2sigma, rest) = cursor.split_at_mut(d.v3rho2sigma as usize * np);
        cursor = rest;
        // Skip v3rho2lapl + v3rho2tau
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v3rho2lapl as usize * np + mgga_d.v3rho2tau as usize * np,
        );
        cursor = rest;
        let (v3rhosigma2, rest) = cursor.split_at_mut(d.v3rhosigma2 as usize * np);
        cursor = rest;
        // Skip v3rhosigmalapl + v3rhosigmatau + v3rholapl2 + v3rholapltau + v3rhotau2
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v3rhosigmalapl as usize * np
                + mgga_d.v3rhosigmatau as usize * np
                + mgga_d.v3rholapl2 as usize * np
                + mgga_d.v3rholapltau as usize * np
                + mgga_d.v3rhotau2 as usize * np,
        );
        cursor = rest;
        let (v3sigma3, rest) = cursor.split_at_mut(d.v3sigma3 as usize * np);
        cursor = rest;
        // Skip remaining order 3 MGGA fields: v3sigma2lapl..v3tau3 (10 fields)
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v3sigma2lapl as usize * np
                + mgga_d.v3sigma2tau as usize * np
                + mgga_d.v3sigmalapl2 as usize * np
                + mgga_d.v3sigmalapltau as usize * np
                + mgga_d.v3sigmatau2 as usize * np
                + mgga_d.v3lapl3 as usize * np
                + mgga_d.v3lapl2tau as usize * np
                + mgga_d.v3lapltau2 as usize * np
                + mgga_d.v3tau3 as usize * np,
        );
        cursor = rest;

        // === Order 4 (35 MGGA fields, 5 of which GGA exposes) ===
        let (v4rho4, rest) = cursor.split_at_mut(d.v4rho4 as usize * np);
        cursor = rest;
        let (v4rho3sigma, rest) = cursor.split_at_mut(d.v4rho3sigma as usize * np);
        cursor = rest;
        // Skip v4rho3lapl + v4rho3tau
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v4rho3lapl as usize * np + mgga_d.v4rho3tau as usize * np,
        );
        cursor = rest;
        let (v4rho2sigma2, rest) = cursor.split_at_mut(d.v4rho2sigma2 as usize * np);
        cursor = rest;
        // Skip v4rho2sigmalapl + v4rho2sigmatau + v4rho2lapl2 + v4rho2lapltau + v4rho2tau2
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v4rho2sigmalapl as usize * np
                + mgga_d.v4rho2sigmatau as usize * np
                + mgga_d.v4rho2lapl2 as usize * np
                + mgga_d.v4rho2lapltau as usize * np
                + mgga_d.v4rho2tau2 as usize * np,
        );
        cursor = rest;
        let (v4rhosigma3, rest) = cursor.split_at_mut(d.v4rhosigma3 as usize * np);
        cursor = rest;
        // Skip v4rhosigma2lapl..v4rhotau3 (9 fields)
        let (_, rest) = cursor.split_at_mut(
            mgga_d.v4rhosigma2lapl as usize * np
                + mgga_d.v4rhosigma2tau as usize * np
                + mgga_d.v4rhosigmalapl2 as usize * np
                + mgga_d.v4rhosigmalapltau as usize * np
                + mgga_d.v4rhosigmatau2 as usize * np
                + mgga_d.v4rholapl3 as usize * np
                + mgga_d.v4rholapl2tau as usize * np
                + mgga_d.v4rholapltau2 as usize * np
                + mgga_d.v4rhotau3 as usize * np,
        );
        cursor = rest;
        let (v4sigma4, _rest) = cursor.split_at_mut(d.v4sigma4 as usize * np);

        GgaScratch {
            zk,
            vrho,
            vsigma,
            v2rho2,
            v2rhosigma,
            v2sigma2,
            v3rho3,
            v3rho2sigma,
            v3rhosigma2,
            v3sigma3,
            v4rho4,
            v4rho3sigma,
            v4rho2sigma2,
            v4rhosigma3,
            v4sigma4,
        }
    }

    /// Get mutable scratch slices for MGGA derivative fields.
    ///
    /// Returns 70 non-overlapping mutable slices into the workspace's
    /// contiguous buffer, covering the full MGGA superset.
    ///
    /// Field ordering follows `Dimensions::total_output_components()` exactly.
    pub fn mgga_scratch_mut(&mut self) -> MggaScratch<'_> {
        let d = &self.dims;
        let np = self.np;
        let buf = self.scratch.as_mut_slice();
        let mut cursor = buf;

        macro_rules! pop {
            ($field:ident) => {{
                let (out, rest) = cursor.split_at_mut(d.$field as usize * np);
                cursor = rest;
                out
            }};
        }

        // Order 0
        let zk = pop!(zk);

        // Order 1 (4 fields)
        let vrho = pop!(vrho);
        let vsigma = pop!(vsigma);
        let vlapl = pop!(vlapl);
        let vtau = pop!(vtau);

        // Order 2 (10 fields)
        let v2rho2 = pop!(v2rho2);
        let v2rhosigma = pop!(v2rhosigma);
        let v2rholapl = pop!(v2rholapl);
        let v2rhotau = pop!(v2rhotau);
        let v2sigma2 = pop!(v2sigma2);
        let v2sigmalapl = pop!(v2sigmalapl);
        let v2sigmatau = pop!(v2sigmatau);
        let v2lapl2 = pop!(v2lapl2);
        let v2lapltau = pop!(v2lapltau);
        let v2tau2 = pop!(v2tau2);

        // Order 3 (20 fields)
        let v3rho3 = pop!(v3rho3);
        let v3rho2sigma = pop!(v3rho2sigma);
        let v3rho2lapl = pop!(v3rho2lapl);
        let v3rho2tau = pop!(v3rho2tau);
        let v3rhosigma2 = pop!(v3rhosigma2);
        let v3rhosigmalapl = pop!(v3rhosigmalapl);
        let v3rhosigmatau = pop!(v3rhosigmatau);
        let v3rholapl2 = pop!(v3rholapl2);
        let v3rholapltau = pop!(v3rholapltau);
        let v3rhotau2 = pop!(v3rhotau2);
        let v3sigma3 = pop!(v3sigma3);
        let v3sigma2lapl = pop!(v3sigma2lapl);
        let v3sigma2tau = pop!(v3sigma2tau);
        let v3sigmalapl2 = pop!(v3sigmalapl2);
        let v3sigmalapltau = pop!(v3sigmalapltau);
        let v3sigmatau2 = pop!(v3sigmatau2);
        let v3lapl3 = pop!(v3lapl3);
        let v3lapl2tau = pop!(v3lapl2tau);
        let v3lapltau2 = pop!(v3lapltau2);
        let v3tau3 = pop!(v3tau3);

        // Order 4 (35 fields)
        let v4rho4 = pop!(v4rho4);
        let v4rho3sigma = pop!(v4rho3sigma);
        let v4rho3lapl = pop!(v4rho3lapl);
        let v4rho3tau = pop!(v4rho3tau);
        let v4rho2sigma2 = pop!(v4rho2sigma2);
        let v4rho2sigmalapl = pop!(v4rho2sigmalapl);
        let v4rho2sigmatau = pop!(v4rho2sigmatau);
        let v4rho2lapl2 = pop!(v4rho2lapl2);
        let v4rho2lapltau = pop!(v4rho2lapltau);
        let v4rho2tau2 = pop!(v4rho2tau2);
        let v4rhosigma3 = pop!(v4rhosigma3);
        let v4rhosigma2lapl = pop!(v4rhosigma2lapl);
        let v4rhosigma2tau = pop!(v4rhosigma2tau);
        let v4rhosigmalapl2 = pop!(v4rhosigmalapl2);
        let v4rhosigmalapltau = pop!(v4rhosigmalapltau);
        let v4rhosigmatau2 = pop!(v4rhosigmatau2);
        let v4rholapl3 = pop!(v4rholapl3);
        let v4rholapl2tau = pop!(v4rholapl2tau);
        let v4rholapltau2 = pop!(v4rholapltau2);
        let v4rhotau3 = pop!(v4rhotau3);
        let v4sigma4 = pop!(v4sigma4);
        let v4sigma3lapl = pop!(v4sigma3lapl);
        let v4sigma3tau = pop!(v4sigma3tau);
        let v4sigma2lapl2 = pop!(v4sigma2lapl2);
        let v4sigma2lapltau = pop!(v4sigma2lapltau);
        let v4sigma2tau2 = pop!(v4sigma2tau2);
        let v4sigmalapl3 = pop!(v4sigmalapl3);
        let v4sigmalapl2tau = pop!(v4sigmalapl2tau);
        let v4sigmalapltau2 = pop!(v4sigmalapltau2);
        let v4sigmatau3 = pop!(v4sigmatau3);
        let v4lapl4 = pop!(v4lapl4);
        let v4lapl3tau = pop!(v4lapl3tau);
        let v4lapl2tau2 = pop!(v4lapl2tau2);
        let v4lapltau3 = pop!(v4lapltau3);
        let v4tau4 = pop!(v4tau4);

        // Suppress unused-cursor warning (cursor consumed by last pop!).
        let _ = cursor;

        MggaScratch {
            zk,
            vrho, vsigma, vlapl, vtau,
            v2rho2, v2rhosigma, v2rholapl, v2rhotau, v2sigma2,
            v2sigmalapl, v2sigmatau, v2lapl2, v2lapltau, v2tau2,
            v3rho3, v3rho2sigma, v3rho2lapl, v3rho2tau, v3rhosigma2,
            v3rhosigmalapl, v3rhosigmatau, v3rholapl2, v3rholapltau, v3rhotau2,
            v3sigma3, v3sigma2lapl, v3sigma2tau, v3sigmalapl2, v3sigmalapltau,
            v3sigmatau2, v3lapl3, v3lapl2tau, v3lapltau2, v3tau3,
            v4rho4, v4rho3sigma, v4rho3lapl, v4rho3tau, v4rho2sigma2,
            v4rho2sigmalapl, v4rho2sigmatau, v4rho2lapl2, v4rho2lapltau, v4rho2tau2,
            v4rhosigma3, v4rhosigma2lapl, v4rhosigma2tau, v4rhosigmalapl2, v4rhosigmalapltau,
            v4rhosigmatau2, v4rholapl3, v4rholapl2tau, v4rholapltau2, v4rhotau3,
            v4sigma4, v4sigma3lapl, v4sigma3tau, v4sigma2lapl2, v4sigma2lapltau,
            v4sigma2tau2, v4sigmalapl3, v4sigmalapl2tau, v4sigmalapltau2, v4sigmatau3,
            v4lapl4, v4lapl3tau, v4lapl2tau2, v4lapltau3, v4tau4,
        }
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
    fn gga_scratch_unpolarized_correct_lengths() {
        let np = 10;
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let s = ws.gga_scratch_mut();
        let d = Dimensions::gga(Spin::Unpolarized);
        assert_eq!(s.zk.len(), d.zk as usize * np);
        assert_eq!(s.vrho.len(), d.vrho as usize * np);
        assert_eq!(s.vsigma.len(), d.vsigma as usize * np);
        assert_eq!(s.v2rho2.len(), d.v2rho2 as usize * np);
        assert_eq!(s.v2rhosigma.len(), d.v2rhosigma as usize * np);
        assert_eq!(s.v2sigma2.len(), d.v2sigma2 as usize * np);
        assert_eq!(s.v3rho3.len(), d.v3rho3 as usize * np);
        assert_eq!(s.v3rho2sigma.len(), d.v3rho2sigma as usize * np);
        assert_eq!(s.v3rhosigma2.len(), d.v3rhosigma2 as usize * np);
        assert_eq!(s.v3sigma3.len(), d.v3sigma3 as usize * np);
        assert_eq!(s.v4rho4.len(), d.v4rho4 as usize * np);
        assert_eq!(s.v4rho3sigma.len(), d.v4rho3sigma as usize * np);
        assert_eq!(s.v4rho2sigma2.len(), d.v4rho2sigma2 as usize * np);
        assert_eq!(s.v4rhosigma3.len(), d.v4rhosigma3 as usize * np);
        assert_eq!(s.v4sigma4.len(), d.v4sigma4 as usize * np);
    }

    #[test]
    fn gga_scratch_polarized_correct_lengths() {
        let np = 10;
        let mut ws = EvaluationWorkspace::new(np, Spin::Polarized);
        let s = ws.gga_scratch_mut();
        let d = Dimensions::gga(Spin::Polarized);
        assert_eq!(s.zk.len(), d.zk as usize * np);
        assert_eq!(s.vrho.len(), d.vrho as usize * np); // 2*np
        assert_eq!(s.vsigma.len(), d.vsigma as usize * np); // 3*np
        assert_eq!(s.v2sigma2.len(), 6 * np);
        assert_eq!(s.v4sigma4.len(), 15 * np);
    }

    #[test]
    fn mgga_scratch_unpolarized_correct_lengths() {
        let np = 10;
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        let s = ws.mgga_scratch_mut();
        let d = Dimensions::mgga(Spin::Unpolarized);
        // Spot-check across orders 0-4.
        assert_eq!(s.zk.len(), d.zk as usize * np);
        assert_eq!(s.vrho.len(), d.vrho as usize * np);
        assert_eq!(s.vlapl.len(), d.vlapl as usize * np);
        assert_eq!(s.vtau.len(), d.vtau as usize * np);
        assert_eq!(s.v2lapl2.len(), d.v2lapl2 as usize * np);
        assert_eq!(s.v2tau2.len(), d.v2tau2 as usize * np);
        assert_eq!(s.v3tau3.len(), d.v3tau3 as usize * np);
        assert_eq!(s.v4tau4.len(), d.v4tau4 as usize * np);
        assert_eq!(s.v4lapl4.len(), d.v4lapl4 as usize * np);
    }

    #[test]
    fn mgga_scratch_polarized_correct_lengths() {
        let np = 5;
        let mut ws = EvaluationWorkspace::new(np, Spin::Polarized);
        let s = ws.mgga_scratch_mut();
        let d = Dimensions::mgga(Spin::Polarized);
        assert_eq!(s.vrho.len(), d.vrho as usize * np);
        assert_eq!(s.v4tau4.len(), d.v4tau4 as usize * np); // 5*np
        assert_eq!(s.v4sigma3tau.len(), 30 * np); // verified per dims spot check
    }

    #[test]
    fn mgga_scratch_total_matches_workspace_size() {
        let np = 7;
        let mut ws = EvaluationWorkspace::new(np, Spin::Polarized);
        let s = ws.mgga_scratch_mut();
        // Sum every field length and confirm == workspace scratch size.
        let mut total = 0usize;
        total += s.zk.len();
        total += s.vrho.len() + s.vsigma.len() + s.vlapl.len() + s.vtau.len();
        total += s.v2rho2.len() + s.v2rhosigma.len() + s.v2rholapl.len() + s.v2rhotau.len()
            + s.v2sigma2.len() + s.v2sigmalapl.len() + s.v2sigmatau.len()
            + s.v2lapl2.len() + s.v2lapltau.len() + s.v2tau2.len();
        total += s.v3rho3.len() + s.v3rho2sigma.len() + s.v3rho2lapl.len() + s.v3rho2tau.len()
            + s.v3rhosigma2.len() + s.v3rhosigmalapl.len() + s.v3rhosigmatau.len()
            + s.v3rholapl2.len() + s.v3rholapltau.len() + s.v3rhotau2.len()
            + s.v3sigma3.len() + s.v3sigma2lapl.len() + s.v3sigma2tau.len()
            + s.v3sigmalapl2.len() + s.v3sigmalapltau.len() + s.v3sigmatau2.len()
            + s.v3lapl3.len() + s.v3lapl2tau.len() + s.v3lapltau2.len() + s.v3tau3.len();
        total += s.v4rho4.len() + s.v4rho3sigma.len() + s.v4rho3lapl.len() + s.v4rho3tau.len()
            + s.v4rho2sigma2.len() + s.v4rho2sigmalapl.len() + s.v4rho2sigmatau.len()
            + s.v4rho2lapl2.len() + s.v4rho2lapltau.len() + s.v4rho2tau2.len()
            + s.v4rhosigma3.len() + s.v4rhosigma2lapl.len() + s.v4rhosigma2tau.len()
            + s.v4rhosigmalapl2.len() + s.v4rhosigmalapltau.len() + s.v4rhosigmatau2.len()
            + s.v4rholapl3.len() + s.v4rholapl2tau.len() + s.v4rholapltau2.len() + s.v4rhotau3.len()
            + s.v4sigma4.len() + s.v4sigma3lapl.len() + s.v4sigma3tau.len()
            + s.v4sigma2lapl2.len() + s.v4sigma2lapltau.len() + s.v4sigma2tau2.len()
            + s.v4sigmalapl3.len() + s.v4sigmalapl2tau.len() + s.v4sigmalapltau2.len()
            + s.v4sigmatau3.len()
            + s.v4lapl4.len() + s.v4lapl3tau.len() + s.v4lapl2tau2.len() + s.v4lapltau3.len()
            + s.v4tau4.len();
        let expected = Dimensions::mgga(Spin::Polarized).total_output_components() * np;
        assert_eq!(total, expected);
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
