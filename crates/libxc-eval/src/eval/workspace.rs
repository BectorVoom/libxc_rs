//! Pre-allocated scratch buffer management for mixed functional evaluation.
//!
//! Mixed/hybrid functionals evaluate multiple auxiliary functionals and combine
//! their results with weights. `EvaluationWorkspace` provides a single contiguous
//! scratch allocation sized for the MGGA superset (D-12), enabling zero-allocation
//! evaluation loops.

use libxc_core::dims::Dimensions;
use libxc_core::model::{DerivativeOrder, Spin};

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
    /// Highest derivative order `scratch` is currently sized for.
    alloc_order: DerivativeOrder,
}

/// Split `n` elements off the front of `cursor`, clamped to what is left.
///
/// Returns an empty slice once the cursor is exhausted rather than panicking,
/// which is what lets a workspace be sized for one derivative order while the
/// accessors still lay out the full field list.
#[inline]
fn take_n<'a>(cursor: &mut &'a mut [f64], n: usize) -> &'a mut [f64] {
    let k = n.min(cursor.len());
    let (head, tail) = std::mem::take(cursor).split_at_mut(k);
    *cursor = tail;
    head
}

impl EvaluationWorkspace {
    /// Create a workspace sized for the MGGA superset at every derivative
    /// order.
    ///
    /// This is the conservative constructor and stays the default so existing
    /// callers keep working. It is also, for almost every caller, far more
    /// memory than the evaluation needs: the superset is 70 doubles per grid
    /// point unpolarized and **767 polarized**, where a polarized GGA `Vxc`
    /// evaluation touches 11. On a million-point grid that is 6.1 GB of
    /// scratch to hold 48 MB of results.
    ///
    /// Prefer [`EvaluationWorkspace::with_order`] when the derivative order is
    /// known, or just let the mixed evaluators call
    /// [`EvaluationWorkspace::ensure_order`], which grows on demand.
    pub fn new(np: usize, spin: Spin) -> Self {
        Self::with_order(np, spin, DerivativeOrder::Lxc)
    }

    /// Create a workspace sized for derivative orders up to `order`.
    ///
    /// The scratch layout is order-major, so everything an evaluation of
    /// `order` can touch lives in the first
    /// `dims.output_components_through(order) * np` elements. Fields above
    /// `order` are handed out as empty slices, which the mixed evaluators
    /// never read because they gate every access on the same `order`.
    pub fn with_order(np: usize, spin: Spin, order: DerivativeOrder) -> Self {
        let dims = Dimensions::mgga(spin);
        let total = dims.output_components_through(order) * np;
        Self {
            scratch: vec![0.0; total],
            np,
            spin,
            dims,
            alloc_order: order,
        }
    }

    /// Grow the scratch so it covers derivative orders up to `order`.
    ///
    /// A no-op when it already does, so a workspace reused across a run
    /// allocates at most once per distinct order and never shrinks. This is
    /// what makes reuse across repeated evaluations allocation-free, which the
    /// project's "reuse workspaces on hot paths" constraint requires.
    pub fn ensure_order(&mut self, order: DerivativeOrder) {
        if order <= self.alloc_order {
            return;
        }
        let needed = self.dims.output_components_through(order) * self.np;
        self.scratch.resize(needed, 0.0);
        self.alloc_order = order;
    }

    /// Highest derivative order the scratch is currently sized for.
    pub fn alloc_order(&self) -> DerivativeOrder {
        self.alloc_order
    }

    /// Scratch capacity in elements. Exposed so a test can assert that a
    /// workspace is the size it claims to be.
    pub fn scratch_len(&self) -> usize {
        self.scratch.len()
    }

    /// Zero every scratch element.
    ///
    /// **Not needed before an auxiliary evaluation, and no longer called
    /// there.** The rayon sweep clears each chunk of every output it is about
    /// to write (`sweep_gga.rs::zero_outputs`, and its LDA/MGGA twins), and
    /// `prepare` clears any buffer the caller supplied that the requested
    /// order does not use. So every element a mixed evaluation reads back has
    /// already been written by the sweep that produced it. Zeroing the whole
    /// superset up front was three extra full passes over 767 doubles per grid
    /// point per auxiliary, all of them dead stores.
    ///
    /// Kept public because it is cheap insurance for a caller doing something
    /// unusual with the raw scratch accessors.
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

        // Walk the MGGA-ordered buffer once, skipping the fields LDA does not
        // expose. `take_n` clamps, so a workspace sized for a lower derivative
        // order yields empty slices for the orders above it rather than
        // panicking -- the mixed evaluators gate every access on the same
        // order the workspace was grown to, so those are never read.
        let mut cursor = self.scratch.as_mut_slice();

        let zk = take_n(&mut cursor, offsets.zk_len);
        let _ = take_n(&mut cursor, offsets.vrho_off - offsets.zk_len);
        let vrho = take_n(&mut cursor, offsets.vrho_len);
        let _ = take_n(
            &mut cursor,
            offsets.v2rho2_off - offsets.vrho_off - offsets.vrho_len,
        );
        let v2rho2 = take_n(&mut cursor, offsets.v2rho2_len);
        let _ = take_n(
            &mut cursor,
            offsets.v3rho3_off - offsets.v2rho2_off - offsets.v2rho2_len,
        );
        let v3rho3 = take_n(&mut cursor, offsets.v3rho3_len);
        let _ = take_n(
            &mut cursor,
            offsets.v4rho4_off - offsets.v3rho3_off - offsets.v3rho3_len,
        );
        let v4rho4 = take_n(&mut cursor, offsets.v4rho4_len);

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
        // `split_at_mut` would panic once the cursor runs out, which it does
        // whenever the workspace was sized for a lower order than the full
        // superset -- so the splits below go through `take_n`, which clamps and
        // hands back an empty slice instead. Callers gate every high-order
        // field on the same `order` the workspace was sized for, so an empty
        // slice is never read.
        let buf = self.scratch.as_mut_slice();
        let mut cursor = buf;

        // === Order 0 ===
        let zk = take_n(&mut cursor, d.zk as usize * np);

        // === Order 1: vrho, vsigma, vlapl, vtau (MGGA layout) ===
        let vrho = take_n(&mut cursor, d.vrho as usize * np);
        let vsigma = take_n(&mut cursor, d.vsigma as usize * np);
        // Skip vlapl + vtau (MGGA-only fields, zero-length for GGA d.)
        let _ = take_n(&mut cursor, mgga_d.vlapl as usize * np + mgga_d.vtau as usize * np);

        // === Order 2 (10 MGGA fields, 3 of which GGA exposes) ===
        let v2rho2 = take_n(&mut cursor, d.v2rho2 as usize * np);
        let v2rhosigma = take_n(&mut cursor, d.v2rhosigma as usize * np);
        // Skip v2rholapl + v2rhotau
        let _ = take_n(&mut cursor, mgga_d.v2rholapl as usize * np + mgga_d.v2rhotau as usize * np,);
        let v2sigma2 = take_n(&mut cursor, d.v2sigma2 as usize * np);
        // Skip v2sigmalapl + v2sigmatau + v2lapl2 + v2lapltau + v2tau2
        let _ = take_n(&mut cursor, mgga_d.v2sigmalapl as usize * np + mgga_d.v2sigmatau as usize * np + mgga_d.v2lapl2 as usize * np + mgga_d.v2lapltau as usize * np + mgga_d.v2tau2 as usize * np,);

        // === Order 3 (20 MGGA fields, 4 of which GGA exposes) ===
        let v3rho3 = take_n(&mut cursor, d.v3rho3 as usize * np);
        let v3rho2sigma = take_n(&mut cursor, d.v3rho2sigma as usize * np);
        // Skip v3rho2lapl + v3rho2tau
        let _ = take_n(&mut cursor, mgga_d.v3rho2lapl as usize * np + mgga_d.v3rho2tau as usize * np,);
        let v3rhosigma2 = take_n(&mut cursor, d.v3rhosigma2 as usize * np);
        // Skip v3rhosigmalapl + v3rhosigmatau + v3rholapl2 + v3rholapltau + v3rhotau2
        let _ = take_n(&mut cursor, mgga_d.v3rhosigmalapl as usize * np + mgga_d.v3rhosigmatau as usize * np + mgga_d.v3rholapl2 as usize * np + mgga_d.v3rholapltau as usize * np + mgga_d.v3rhotau2 as usize * np,);
        let v3sigma3 = take_n(&mut cursor, d.v3sigma3 as usize * np);
        // Skip remaining order 3 MGGA fields: v3sigma2lapl..v3tau3 (10 fields)
        let _ = take_n(&mut cursor, mgga_d.v3sigma2lapl as usize * np + mgga_d.v3sigma2tau as usize * np + mgga_d.v3sigmalapl2 as usize * np + mgga_d.v3sigmalapltau as usize * np + mgga_d.v3sigmatau2 as usize * np + mgga_d.v3lapl3 as usize * np + mgga_d.v3lapl2tau as usize * np + mgga_d.v3lapltau2 as usize * np + mgga_d.v3tau3 as usize * np,);

        // === Order 4 (35 MGGA fields, 5 of which GGA exposes) ===
        let v4rho4 = take_n(&mut cursor, d.v4rho4 as usize * np);
        let v4rho3sigma = take_n(&mut cursor, d.v4rho3sigma as usize * np);
        // Skip v4rho3lapl + v4rho3tau
        let _ = take_n(&mut cursor, mgga_d.v4rho3lapl as usize * np + mgga_d.v4rho3tau as usize * np,);
        let v4rho2sigma2 = take_n(&mut cursor, d.v4rho2sigma2 as usize * np);
        // Skip v4rho2sigmalapl + v4rho2sigmatau + v4rho2lapl2 + v4rho2lapltau + v4rho2tau2
        let _ = take_n(&mut cursor, mgga_d.v4rho2sigmalapl as usize * np + mgga_d.v4rho2sigmatau as usize * np + mgga_d.v4rho2lapl2 as usize * np + mgga_d.v4rho2lapltau as usize * np + mgga_d.v4rho2tau2 as usize * np,);
        let v4rhosigma3 = take_n(&mut cursor, d.v4rhosigma3 as usize * np);
        // Skip v4rhosigma2lapl..v4rhotau3 (9 fields)
        let _ = take_n(&mut cursor, mgga_d.v4rhosigma2lapl as usize * np + mgga_d.v4rhosigma2tau as usize * np + mgga_d.v4rhosigmalapl2 as usize * np + mgga_d.v4rhosigmalapltau as usize * np + mgga_d.v4rhosigmatau2 as usize * np + mgga_d.v4rholapl3 as usize * np + mgga_d.v4rholapl2tau as usize * np + mgga_d.v4rholapltau2 as usize * np + mgga_d.v4rhotau3 as usize * np,);
        let v4sigma4 = take_n(&mut cursor, d.v4sigma4 as usize * np);

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
                let out = take_n(&mut cursor, d.$field as usize * np);
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
    use libxc_core::dims::Dimensions;
    use libxc_core::model::{DerivativeOrder, Spin};

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

    /// A workspace sized for a low derivative order must hand out empty
    /// slices for the orders above it, not panic.
    ///
    /// Regression guard: the accessors used to carve the buffer with
    /// `split_at_mut`, which panics the moment the cursor runs out. That is
    /// reachable from `evaluate_mixed_gga`, whose LDA-auxiliary branch calls
    /// `lda_scratch_mut()` on whatever workspace the caller supplied -- so a
    /// GGA hybrid with an LDA auxiliary (B3LYP has two) plus a right-sized
    /// workspace would have gone straight through it.
    #[test]
    fn low_order_workspace_yields_empty_high_order_slices() {
        let np = 32;
        for spin in [Spin::Unpolarized, Spin::Polarized] {
            let mut ws = EvaluationWorkspace::with_order(np, spin, DerivativeOrder::Vxc);
            let d = Dimensions::mgga(spin);
            assert_eq!(
                ws.scratch_len(),
                d.output_components_through(DerivativeOrder::Vxc) * np
            );

            let lda = ws.lda_scratch_mut();
            assert_eq!(lda.zk.len(), Dimensions::lda(spin).zk as usize * np);
            assert_eq!(lda.vrho.len(), Dimensions::lda(spin).vrho as usize * np);
            assert!(lda.v2rho2.is_empty(), "order 2 is above the allocation");
            assert!(lda.v3rho3.is_empty());
            assert!(lda.v4rho4.is_empty());

            let gga = ws.gga_scratch_mut();
            assert_eq!(gga.vsigma.len(), Dimensions::gga(spin).vsigma as usize * np);
            assert!(gga.v2sigma2.is_empty());
            assert!(gga.v4sigma4.is_empty());

            let mg = ws.mgga_scratch_mut();
            assert_eq!(mg.vtau.len(), d.vtau as usize * np);
            assert!(mg.v2tau2.is_empty());
        }
    }

    /// `ensure_order` grows and never shrinks, so a reused workspace pays for
    /// each distinct order at most once.
    #[test]
    fn ensure_order_grows_monotonically() {
        let np = 16;
        let d = Dimensions::mgga(Spin::Polarized);
        let mut ws = EvaluationWorkspace::with_order(np, Spin::Polarized, DerivativeOrder::Exc);
        assert_eq!(ws.scratch_len(), d.output_components_through(DerivativeOrder::Exc) * np);

        ws.ensure_order(DerivativeOrder::Fxc);
        let after_fxc = ws.scratch_len();
        assert_eq!(after_fxc, d.output_components_through(DerivativeOrder::Fxc) * np);

        // Asking for less must not shrink it -- that would make a reused
        // workspace reallocate on every alternation.
        ws.ensure_order(DerivativeOrder::Vxc);
        assert_eq!(ws.scratch_len(), after_fxc);
        assert_eq!(ws.alloc_order(), DerivativeOrder::Fxc);

        ws.ensure_order(DerivativeOrder::Lxc);
        assert_eq!(ws.scratch_len(), d.total_output_components() * np);
    }

    /// The default constructor still covers everything, so existing callers
    /// see no change.
    #[test]
    fn new_is_the_full_superset() {
        let np = 8;
        for spin in [Spin::Unpolarized, Spin::Polarized] {
            let ws = EvaluationWorkspace::new(np, spin);
            assert_eq!(
                ws.scratch_len(),
                Dimensions::mgga(spin).total_output_components() * np
            );
            assert_eq!(ws.alloc_order(), DerivativeOrder::Lxc);
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
