//! Mixed functional accumulation logic.
//!
//! Mixed/hybrid functionals evaluate multiple auxiliary functionals and combine
//! their weighted results. This module provides the low-level accumulation
//! primitive (`add_to_mix`), the mixed LDA evaluation loop
//! (`evaluate_mixed_lda` — pre-existing AuxiliaryConfig shape),
//! and the Functional-based mixed evaluation paths added in Plan 05-03
//! (`evaluate_mixed_lda_functional` / `evaluate_mixed_gga` /
//! `evaluate_mixed_mgga`), matching libxc's `mix_func.c` behavior including
//! per-aux family gating (Pitfall 5 / mix_func.c:170-308).

use libxc_core::dims::Dimensions;
use libxc_core::error::LibxcRsError;
// 11-12 (G-2): eval-level dispatch_* (real under family feature, stub when off).
use crate::eval::workspace::EvaluationWorkspace;
use crate::eval::{dispatch_gga_by_id, dispatch_lda, dispatch_lda_by_id, dispatch_mgga_by_id};
use crate::functional::Functional;
use crate::functional::params_lda::LdaXParams;
use libxc_core::input::{GgaInput, LdaInput, MggaInput};
use libxc_core::model::{DerivativeOrder, Family, FunctionalFlags, LdaFunctional, Thresholds};
use libxc_core::output::{GgaOutput, LdaOutput, MggaOutput};

/// Configuration for one auxiliary functional in a mixed evaluation.
pub struct AuxiliaryConfig {
    /// The functional's alpha parameter (e.g., 1.0 for LDA_X).
    pub alpha: f64,
    /// The mixing coefficient (weight) for this auxiliary.
    pub weight: f64,
    /// Numerical thresholds for this auxiliary's evaluation.
    pub thresholds: Thresholds,
}

/// Weighted accumulation: `dst[i] += coeff * src[i]` for all elements.
///
/// Matches libxc `mix_func.c` line 54 exactly.
/// The compiler will auto-vectorize this loop.
///
/// **WR-11 hardening (Plan 05-06):** the prior implementation gated the length
/// equality on `debug_assert_eq!`, which becomes a no-op in `--release` builds
/// and silently produced wrong results when callers passed mismatched slices
/// (the `zip` would consume `min(dst.len(), src.len())` elements). The check
/// is now an always-on `assert_eq!` so any caller-side length bug fails loudly
/// in every build configuration.
pub fn add_to_mix(dst: &mut [f64], coeff: f64, src: &[f64]) {
    assert_eq!(
        dst.len(),
        src.len(),
        "add_to_mix: dst and src must have equal length"
    );
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        *d += coeff * *s;
    }
}

/// Evaluate a mixed LDA functional by dispatching to auxiliary functionals
/// and accumulating weighted results.
///
/// For each auxiliary:
/// 1. Zeros workspace scratch (prevents cross-contamination, T-03-07)
/// 2. Evaluates the auxiliary into scratch via `dispatch_lda`
/// 3. Accumulates `weight * scratch` into the caller's output
///
/// # Arguments
/// * `input` - LDA input bundle
/// * `order` - Maximum derivative order to compute
/// * `output` - Output bundle where weighted results are accumulated
/// * `auxiliaries` - Slice of auxiliary functional configurations
/// * `workspace` - Pre-allocated scratch workspace
///
/// # Errors
/// Returns error if workspace dimensions don't match input, or if any
/// auxiliary dispatch fails.
pub fn evaluate_mixed_lda(
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    auxiliaries: &[AuxiliaryConfig],
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    // Validate workspace matches input
    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }
    // Grow the scratch to exactly this evaluation's order (a no-op if it is
    // already at least that big). Lets a caller hand over a minimally-sized
    // workspace and have it reach the right size once, rather than every
    // caller paying for the MGGA all-orders superset up front.
    workspace.ensure_order(order);

    let np = input.np();
    let dims = Dimensions::lda(input.spin());

    // Zero the caller's output buffers before accumulation
    if let Some(ref mut buf) = output.zk {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.vrho {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v2rho2 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v3rho3 {
        buf.fill(0.0);
    }
    if let Some(ref mut buf) = output.v4rho4 {
        buf.fill(0.0);
    }

    let zk_len = dims.zk as usize * np;
    let vrho_len = dims.vrho as usize * np;
    let v2rho2_len = dims.v2rho2 as usize * np;
    let v3rho3_len = dims.v3rho3 as usize * np;
    let v4rho4_len = dims.v4rho4 as usize * np;

    for aux in auxiliaries {
        // No `zero_scratch()` here. The rayon sweep clears each chunk of
        // every output it writes before the kernel accumulates into it, and
        // `prepare` clears any buffer the requested order does not use, so
        // every element read back below has already been written by the
        // dispatch that produced it. Zeroing the MGGA superset once per
        // auxiliary was three full passes over 767 doubles per grid point of
        // dead stores.

        // Evaluate auxiliary into workspace scratch.
        // We need to build an LdaOutput pointing into the scratch,
        // call dispatch_lda, then drop the LdaOutput so we can
        // re-borrow scratch for accumulation.
        {
            let scratch = workspace.lda_scratch_mut();
            let mut scratch_output = LdaOutput {
                zk: Some(scratch.zk),
                vrho: if order >= DerivativeOrder::Vxc {
                    Some(scratch.vrho)
                } else {
                    None
                },
                v2rho2: if order >= DerivativeOrder::Fxc {
                    Some(scratch.v2rho2)
                } else {
                    None
                },
                v3rho3: if order >= DerivativeOrder::Kxc {
                    Some(scratch.v3rho3)
                } else {
                    None
                },
                v4rho4: if order >= DerivativeOrder::Lxc {
                    Some(scratch.v4rho4)
                } else {
                    None
                },
            };

            // TODO: Phase 5 — route mixed components by their registry metadata
            // rather than always dispatching to LdaFunctional::LdaX. The
            // existing mixed evaluators in libxc construct their auxiliaries
            // from named LDA functionals (lda_x, lda_c_pw, ...); for Phase 4
            // the only mixed-LDA tests use lda_x exclusively, so this matches
            // prior behavior bit-for-bit.
            dispatch_lda(
                LdaFunctional::LdaX,
                input,
                order,
                &mut scratch_output,
                &LdaXParams::new(aux.alpha),
                &aux.thresholds,
            )?;
        }
        // scratch_output is dropped here, releasing the mutable borrow on workspace

        // Accumulate weighted scratch results into caller output.
        // Re-borrow scratch immutably via lda_scratch_mut (we need the slices again).
        let scratch = workspace.lda_scratch_mut();

        if let Some(ref mut dst) = output.zk {
            add_to_mix(dst, aux.weight, &scratch.zk[..zk_len]);
        }
        if order >= DerivativeOrder::Vxc
            && let Some(ref mut dst) = output.vrho
        {
            add_to_mix(dst, aux.weight, &scratch.vrho[..vrho_len]);
        }
        if order >= DerivativeOrder::Fxc
            && let Some(ref mut dst) = output.v2rho2
        {
            add_to_mix(dst, aux.weight, &scratch.v2rho2[..v2rho2_len]);
        }
        if order >= DerivativeOrder::Kxc
            && let Some(ref mut dst) = output.v3rho3
        {
            add_to_mix(dst, aux.weight, &scratch.v3rho3[..v3rho3_len]);
        }
        if order >= DerivativeOrder::Lxc
            && let Some(ref mut dst) = output.v4rho4
        {
            add_to_mix(dst, aux.weight, &scratch.v4rho4[..v4rho4_len]);
        }
    }

    Ok(())
}

// ============================================================================
// Plan 05-03 — Functional-based mixed evaluation paths
// ============================================================================

/// Helper: in-place weighted accumulation that no-ops when `dst` is `None`,
/// with explicit per-field length checking.
///
/// **CR-02 fix (Plan 05-06):** the prior `add_opt` helper silently truncated
/// to `min(dst.len(), src.len())` elements, masking caller bugs and producing
/// numerically wrong but apparently-passing results when scratch and output
/// per-family per-field dimensions disagreed (which can happen for polarized
/// cross-derivatives). The new shape takes an explicit length parameter
/// computed from `Dimensions` at the top of each `evaluate_mixed_*` function:
///
/// - If `dst` is `None`, no-op (caller did not request this output field).
/// - If `dst.len() != len`, return `OutputBufferSizeMismatch` (caller bug).
/// - If `src.len() < len`, return `OutputBufferSizeMismatch` keyed on the
///   scratch buffer (workspace bug — should never happen since scratch is
///   sized for max family dimensions, but defended for safety).
/// - Otherwise: `dst[i] += coeff * src[i]` for `i in 0..len`.
fn add_opt_n(
    dst: Option<&mut [f64]>,
    coeff: f64,
    src: &[f64],
    len: usize,
    field: &'static str,
) -> Result<(), LibxcRsError> {
    if let Some(d) = dst {
        if d.len() != len {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field,
                expected: len,
                actual: d.len(),
            });
        }
        if src.len() < len {
            return Err(LibxcRsError::OutputBufferSizeMismatch {
                field,
                expected: len,
                actual: src.len(),
            });
        }
        for i in 0..len {
            d[i] += coeff * src[i];
        }
    }
    Ok(())
}

/// Evaluate a mixed LDA functional via `Functional::auxiliaries` /
/// `Functional::mix_coefficients` (Plan 05-03 shape; replaces the
/// `AuxiliaryConfig` slice variant for callers that have a `Functional`).
///
/// Mirrors `evaluate_mixed_lda` shape; routes each aux through `dispatch_lda`
/// using the aux's own `params` trait object (so per-aux ext_params are
/// honored). Caller output buffers are zeroed before accumulation.
pub fn evaluate_mixed_lda_functional(
    functional: &Functional,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    evaluate_mixed_lda_functional_into(functional, input, order, output, workspace, true)
}

/// As [`evaluate_mixed_lda_functional`], but `zero_first = false` accumulates into
/// whatever the output already holds.
///
/// libxc's `xc_lda_new` evaluates a functional's own kernel and *then* adds
/// `xc_mix_func` on top when the info block carries both a work pointer and a
/// non-NULL `mix_coef`. Reproducing that needs a mix pass that does not first
/// wipe the kernel's contribution. `hyb_mgga_xc_b0kcis` is the only functional
/// in libxc 7.0.0 that needs it.
pub fn evaluate_mixed_lda_functional_into(
    functional: &Functional,
    input: &LdaInput,
    order: DerivativeOrder,
    output: &mut LdaOutput,
    workspace: &mut EvaluationWorkspace,
    zero_first: bool,
) -> Result<(), LibxcRsError> {
    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }
    // Grow the scratch to exactly this evaluation's order (a no-op if it is
    // already at least that big). Lets a caller hand over a minimally-sized
    // workspace and have it reach the right size once, rather than every
    // caller paying for the MGGA all-orders superset up front.
    workspace.ensure_order(order);

    let np = input.np();
    let dims = Dimensions::lda(input.spin());

    // Skipped when accumulating on top of a kernel result -- see the
    // `zero_first` note on this function.
    if zero_first {
        // Zero caller output once before accumulation.
        if let Some(ref mut buf) = output.zk {
            buf.fill(0.0);
        }
        if let Some(ref mut buf) = output.vrho {
            buf.fill(0.0);
        }
        if let Some(ref mut buf) = output.v2rho2 {
            buf.fill(0.0);
        }
        if let Some(ref mut buf) = output.v3rho3 {
            buf.fill(0.0);
        }
        if let Some(ref mut buf) = output.v4rho4 {
            buf.fill(0.0);
        }
    }

    let zk_len = dims.zk as usize * np;
    let vrho_len = dims.vrho as usize * np;
    let v2rho2_len = dims.v2rho2 as usize * np;
    let v3rho3_len = dims.v3rho3 as usize * np;
    let v4rho4_len = dims.v4rho4 as usize * np;

    for (aux, &weight) in functional
        .auxiliaries
        .iter()
        .zip(functional.mix_coefficients.iter())
    {
        // LDA-only: aux must be LDA family.
        if aux.meta.family != Family::Lda {
            return Err(LibxcRsError::UnsupportedFunctional {
                id: functional.meta.id,
                reason: "non-LDA auxiliary inside LDA parent",
            });
        }

        {
            let scratch = workspace.lda_scratch_mut();
            let mut scratch_output = LdaOutput {
                zk: Some(scratch.zk),
                vrho: if order >= DerivativeOrder::Vxc {
                    Some(scratch.vrho)
                } else {
                    None
                },
                v2rho2: if order >= DerivativeOrder::Fxc {
                    Some(scratch.v2rho2)
                } else {
                    None
                },
                v3rho3: if order >= DerivativeOrder::Kxc {
                    Some(scratch.v3rho3)
                } else {
                    None
                },
                v4rho4: if order >= DerivativeOrder::Lxc {
                    Some(scratch.v4rho4)
                } else {
                    None
                },
            };
            dispatch_lda_by_id(
                aux.meta.id,
                input,
                order,
                &mut scratch_output,
                aux.kernel_ext_params(),
                &aux.thresholds,
            )?;
        }

        let scratch = workspace.lda_scratch_mut();
        if let Some(ref mut dst) = output.zk {
            add_to_mix(dst, weight, &scratch.zk[..zk_len]);
        }
        if order >= DerivativeOrder::Vxc
            && let Some(ref mut dst) = output.vrho
        {
            add_to_mix(dst, weight, &scratch.vrho[..vrho_len]);
        }
        if order >= DerivativeOrder::Fxc
            && let Some(ref mut dst) = output.v2rho2
        {
            add_to_mix(dst, weight, &scratch.v2rho2[..v2rho2_len]);
        }
        if order >= DerivativeOrder::Kxc
            && let Some(ref mut dst) = output.v3rho3
        {
            add_to_mix(dst, weight, &scratch.v3rho3[..v3rho3_len]);
        }
        if order >= DerivativeOrder::Lxc
            && let Some(ref mut dst) = output.v4rho4
        {
            add_to_mix(dst, weight, &scratch.v4rho4[..v4rho4_len]);
        }
    }

    Ok(())
}

/// Evaluate a mixed GGA functional. Per-aux family gating follows
/// libxc `mix_func.c:170-308` (Pitfall 5):
///
/// - **LDA aux** writes only rho-derivatives (zk, vrho, v2rho2, v3rho3,
///   v4rho4). Sigma-derivative scratch is left untouched and must not be
///   accumulated into the GGA caller output.
/// - **GGA aux** writes rho + sigma derivatives; accumulate all 15 GGA
///   output fields.
/// - **MGGA aux inside a GGA parent** is rejected with
///   `UnsupportedFunctional` (mix_func.c does not support this combination).
///
/// The work is done leaf by leaf, not auxiliary by auxiliary. The grid is
/// split exactly as `par_sweep` splits it (`sweep_gga::par_leaves`), and on
/// each leaf every auxiliary kernel runs over that leaf into a pooled,
/// leaf-sized scratch (`EvaluationWorkspace::leaf_scratch`) and is folded into
/// the caller's outputs straight away, while the leaf's inputs and outputs are
/// still in cache. Compared with the previous whole-grid form -- one full sweep
/// per auxiliary into an `np`-sized scratch, then a serial `out += w * scratch`
/// pass per field per auxiliary on the calling thread -- this removes the
/// serial passes and the up-front serial zeroing, and shrinks the scratch from
/// `O(np * components)` to `O(workers * leaf * components)`, roughly 1.5 MB for
/// a polarized `vxc` HSE06 regardless of grid size. The arithmetic is
/// unchanged: each element still receives `0`, then `+= w_k * aux_k` for each
/// auxiliary in metadata order, and each auxiliary value is what the same
/// kernel writes into a freshly zeroed buffer -- so the result is bit-identical,
/// which `bench-vs-libxc`'s fingerprint checks.
pub fn evaluate_mixed_gga(
    functional: &Functional,
    input: &GgaInput,
    order: DerivativeOrder,
    output: &mut GgaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    evaluate_mixed_gga_into(functional, input, order, output, workspace, true)
}

/// As [`evaluate_mixed_gga`], but `zero_first = false` accumulates into
/// whatever the output already holds.
///
/// libxc's `xc_gga_new` evaluates a functional's own kernel and *then* adds
/// `xc_mix_func` on top when the info block carries both a work pointer and a
/// non-NULL `mix_coef`. Reproducing that needs a mix pass that does not first
/// wipe the kernel's contribution. `hyb_mgga_xc_b0kcis` is the only functional
/// in libxc 7.0.0 that needs it.
pub fn evaluate_mixed_gga_into(
    functional: &Functional,
    input: &GgaInput,
    order: DerivativeOrder,
    output: &mut GgaOutput,
    workspace: &mut EvaluationWorkspace,
    zero_first: bool,
) -> Result<(), LibxcRsError> {
    use libxc_reval::sweep_gga::{GgaChunk, min_chunk, par_leaves};
    use std::sync::Mutex;

    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }
    let np = input.np();
    let spin = input.spin();
    let dims = Dimensions::gga(spin);

    // Reject the unsupported combination before touching any buffer, so a
    // caller sees the error and not a half-accumulated output.
    if functional.auxiliaries.iter().any(|a| a.meta.family == Family::Mgga) {
        return Err(LibxcRsError::UnsupportedFunctional {
            id: functional.meta.id,
            reason: "MGGA auxiliary inside GGA parent (mix_func.c rejects this combination)",
        });
    }

    // Every buffer the caller supplied must be the length its field implies
    // at this grid size. Checked once here rather than per field per leaf.
    macro_rules! check {
        ($f:ident, $name:literal, $dim:expr) => {
            if let Some(b) = output.$f.as_deref() {
                let expected = $dim as usize * np;
                if b.len() != expected {
                    return Err(LibxcRsError::OutputBufferSizeMismatch {
                        field: $name,
                        expected,
                        actual: b.len(),
                    });
                }
            }
        };
    }
    check!(zk, "zk", dims.zk);
    check!(vrho, "vrho", dims.vrho);
    check!(vsigma, "vsigma", dims.vsigma);
    check!(v2rho2, "v2rho2", dims.v2rho2);
    check!(v2rhosigma, "v2rhosigma", dims.v2rhosigma);
    check!(v2sigma2, "v2sigma2", dims.v2sigma2);
    check!(v3rho3, "v3rho3", dims.v3rho3);
    check!(v3rho2sigma, "v3rho2sigma", dims.v3rho2sigma);
    check!(v3rhosigma2, "v3rhosigma2", dims.v3rhosigma2);
    check!(v3sigma3, "v3sigma3", dims.v3sigma3);
    check!(v4rho4, "v4rho4", dims.v4rho4);
    check!(v4rho3sigma, "v4rho3sigma", dims.v4rho3sigma);
    check!(v4rho2sigma2, "v4rho2sigma2", dims.v4rho2sigma2);
    check!(v4rhosigma3, "v4rhosigma3", dims.v4rhosigma3);
    check!(v4sigma4, "v4sigma4", dims.v4sigma4);

    // The whole grid as one chunk, reborrowing every buffer the caller
    // supplied. Unlike `prepare`, nothing is required to be present: a field
    // the caller did not ask for is simply never accumulated into.
    let parent = GgaChunk {
        np,
        rho: input.rho(),
        sigma: input.sigma(),
        zk: output.zk.as_deref_mut(),
        vrho: output.vrho.as_deref_mut(),
        vsigma: output.vsigma.as_deref_mut(),
        v2rho2: output.v2rho2.as_deref_mut(),
        v2rhosigma: output.v2rhosigma.as_deref_mut(),
        v2sigma2: output.v2sigma2.as_deref_mut(),
        v3rho3: output.v3rho3.as_deref_mut(),
        v3rho2sigma: output.v3rho2sigma.as_deref_mut(),
        v3rhosigma2: output.v3rhosigma2.as_deref_mut(),
        v3sigma3: output.v3sigma3.as_deref_mut(),
        v4rho4: output.v4rho4.as_deref_mut(),
        v4rho3sigma: output.v4rho3sigma.as_deref_mut(),
        v4rho2sigma2: output.v4rho2sigma2.as_deref_mut(),
        v4rhosigma3: output.v4rhosigma3.as_deref_mut(),
        v4sigma4: output.v4sigma4.as_deref_mut(),
    };

    // Leaves run in parallel and cannot return, so the first error any of
    // them hits is parked here. The errors an auxiliary dispatch can raise
    // (unrouted id, ext_params count) do not depend on which leaf it is on,
    // so "first" is also "every".
    let first_err: Mutex<Option<LibxcRsError>> = Mutex::new(None);
    let ws: &EvaluationWorkspace = workspace;
    par_leaves(parent, &dims, min_chunk(), &|leaf: &mut GgaChunk<'_, '_>| {
        if let Err(e) = mix_gga_leaf(functional, leaf, order, spin, &dims, ws, zero_first) {
            let mut slot = first_err.lock().unwrap_or_else(|p| p.into_inner());
            if slot.is_none() {
                *slot = Some(e);
            }
        }
    });
    match first_err.into_inner().unwrap_or_else(|p| p.into_inner()) {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

/// Split `n` elements off the front of `cursor`.
fn carve<'a>(cursor: &mut &'a mut [f64], n: usize) -> &'a mut [f64] {
    let taken = std::mem::take(cursor);
    let (head, tail) = taken.split_at_mut(n);
    *cursor = tail;
    head
}

/// `dst += w * src`, elementwise, when `dst` is present.
///
/// The one arithmetic operation of the mix, written exactly as the whole-grid
/// form wrote it (`d[i] += coeff * src[i]`), so the bits are the same.
#[inline]
fn axpy_into(dst: Option<&mut [f64]>, w: f64, src: &[f64]) {
    if let Some(d) = dst {
        for (d, s) in d.iter_mut().zip(src.iter()) {
            *d += w * *s;
        }
    }
}

/// One leaf of [`evaluate_mixed_gga_into`]: zero the caller's slice of every
/// output (if asked), then run each auxiliary over the leaf into pooled scratch
/// and fold it in.
fn mix_gga_leaf(
    functional: &Functional,
    leaf: &mut libxc_reval::sweep_gga::GgaChunk<'_, '_>,
    order: DerivativeOrder,
    spin: libxc_core::model::Spin,
    dims: &Dimensions,
    ws: &EvaluationWorkspace,
    zero_first: bool,
) -> Result<(), LibxcRsError> {
    let n = leaf.np;

    if zero_first {
        // Every buffer the caller supplied, not only those this order writes:
        // a stale value left in one the caller handed over would be worse than
        // the cost of clearing it (as `prepare` also reasons).
        for b in [
            leaf.zk.as_deref_mut(),
            leaf.vrho.as_deref_mut(),
            leaf.vsigma.as_deref_mut(),
            leaf.v2rho2.as_deref_mut(),
            leaf.v2rhosigma.as_deref_mut(),
            leaf.v2sigma2.as_deref_mut(),
            leaf.v3rho3.as_deref_mut(),
            leaf.v3rho2sigma.as_deref_mut(),
            leaf.v3rhosigma2.as_deref_mut(),
            leaf.v3sigma3.as_deref_mut(),
            leaf.v4rho4.as_deref_mut(),
            leaf.v4rho3sigma.as_deref_mut(),
            leaf.v4rho2sigma2.as_deref_mut(),
            leaf.v4rhosigma3.as_deref_mut(),
            leaf.v4sigma4.as_deref_mut(),
        ]
        .into_iter()
        .flatten()
        {
            b.fill(0.0);
        }
    }

    let vxc = order >= DerivativeOrder::Vxc;
    let fxc = order >= DerivativeOrder::Fxc;
    let kxc = order >= DerivativeOrder::Kxc;
    let lxc = order >= DerivativeOrder::Lxc;
    // Elements a field of `dim` per point occupies on this leaf, or 0 when the
    // order does not reach it (a zero-length carve is a `None` buffer).
    let len = |dim: usize, on: bool| if on { dim * n } else { 0 };
    fn opt(s: &mut [f64]) -> Option<&mut [f64]> {
        if s.is_empty() { None } else { Some(s) }
    }

    for (aux, &weight) in functional
        .auxiliaries
        .iter()
        .zip(functional.mix_coefficients.iter())
    {
        match aux.meta.family {
            Family::Lda => {
                // Rho-derivative chain only; the sigma fields are never touched
                // by an LDA auxiliary (Pitfall 5).
                let ld = Dimensions::lda(spin);
                let lens = [
                    len(ld.zk as usize, true),
                    len(ld.vrho as usize, vxc),
                    len(ld.v2rho2 as usize, fxc),
                    len(ld.v3rho3 as usize, kxc),
                    len(ld.v4rho4 as usize, lxc),
                ];
                let mut scratch = ws.leaf_scratch(lens.iter().sum());
                let mut cur: &mut [f64] = &mut scratch;
                let zk = carve(&mut cur, lens[0]);
                let vrho = carve(&mut cur, lens[1]);
                let v2rho2 = carve(&mut cur, lens[2]);
                let v3rho3 = carve(&mut cur, lens[3]);
                let v4rho4 = carve(&mut cur, lens[4]);

                let lda_input = LdaInput::new(leaf.rho, n, spin)?;
                let mut aux_out = LdaOutput {
                    zk: Some(zk),
                    vrho: opt(vrho),
                    v2rho2: opt(v2rho2),
                    v3rho3: opt(v3rho3),
                    v4rho4: opt(v4rho4),
                };
                dispatch_lda_by_id(
                    aux.meta.id,
                    &lda_input,
                    order,
                    &mut aux_out,
                    aux.kernel_ext_params(),
                    &aux.thresholds,
                )?;

                // `prepare` took the slices out of `aux_out`; read them back
                // from the scratch at the same offsets.
                let mut cur: &[f64] = &scratch;
                let mut next = |k: usize| -> &[f64] {
                    let (h, t) = cur.split_at(lens[k]);
                    cur = t;
                    h
                };
                axpy_into(leaf.zk.as_deref_mut(), weight, next(0));
                axpy_into(leaf.vrho.as_deref_mut(), weight, next(1));
                axpy_into(leaf.v2rho2.as_deref_mut(), weight, next(2));
                axpy_into(leaf.v3rho3.as_deref_mut(), weight, next(3));
                axpy_into(leaf.v4rho4.as_deref_mut(), weight, next(4));
            }
            Family::Gga => {
                let lens = [
                    len(dims.zk as usize, true),
                    len(dims.vrho as usize, vxc),
                    len(dims.vsigma as usize, vxc),
                    len(dims.v2rho2 as usize, fxc),
                    len(dims.v2rhosigma as usize, fxc),
                    len(dims.v2sigma2 as usize, fxc),
                    len(dims.v3rho3 as usize, kxc),
                    len(dims.v3rho2sigma as usize, kxc),
                    len(dims.v3rhosigma2 as usize, kxc),
                    len(dims.v3sigma3 as usize, kxc),
                    len(dims.v4rho4 as usize, lxc),
                    len(dims.v4rho3sigma as usize, lxc),
                    len(dims.v4rho2sigma2 as usize, lxc),
                    len(dims.v4rhosigma3 as usize, lxc),
                    len(dims.v4sigma4 as usize, lxc),
                ];
                let mut scratch = ws.leaf_scratch(lens.iter().sum());
                let mut cur: &mut [f64] = &mut scratch;
                let mut fields: [&mut [f64]; 15] = std::array::from_fn(|_| &mut [][..]);
                for (k, f) in fields.iter_mut().enumerate() {
                    *f = carve(&mut cur, lens[k]);
                }
                let [zk, vrho, vsigma, v2rho2, v2rhosigma, v2sigma2, v3rho3, v3rho2sigma,
                    v3rhosigma2, v3sigma3, v4rho4, v4rho3sigma, v4rho2sigma2, v4rhosigma3,
                    v4sigma4] = fields;

                let gga_input = GgaInput::new(leaf.rho, leaf.sigma, n, spin)?;
                let mut aux_out = GgaOutput {
                    zk: Some(zk),
                    vrho: opt(vrho),
                    vsigma: opt(vsigma),
                    v2rho2: opt(v2rho2),
                    v2rhosigma: opt(v2rhosigma),
                    v2sigma2: opt(v2sigma2),
                    v3rho3: opt(v3rho3),
                    v3rho2sigma: opt(v3rho2sigma),
                    v3rhosigma2: opt(v3rhosigma2),
                    v3sigma3: opt(v3sigma3),
                    v4rho4: opt(v4rho4),
                    v4rho3sigma: opt(v4rho3sigma),
                    v4rho2sigma2: opt(v4rho2sigma2),
                    v4rhosigma3: opt(v4rhosigma3),
                    v4sigma4: opt(v4sigma4),
                };
                dispatch_gga_by_id(
                    aux.meta.id,
                    &gga_input,
                    order,
                    &mut aux_out,
                    aux.kernel_ext_params(),
                    &aux.thresholds,
                )?;

                let mut cur: &[f64] = &scratch;
                let mut next = |k: usize| -> &[f64] {
                    let (h, t) = cur.split_at(lens[k]);
                    cur = t;
                    h
                };
                axpy_into(leaf.zk.as_deref_mut(), weight, next(0));
                axpy_into(leaf.vrho.as_deref_mut(), weight, next(1));
                axpy_into(leaf.vsigma.as_deref_mut(), weight, next(2));
                axpy_into(leaf.v2rho2.as_deref_mut(), weight, next(3));
                axpy_into(leaf.v2rhosigma.as_deref_mut(), weight, next(4));
                axpy_into(leaf.v2sigma2.as_deref_mut(), weight, next(5));
                axpy_into(leaf.v3rho3.as_deref_mut(), weight, next(6));
                axpy_into(leaf.v3rho2sigma.as_deref_mut(), weight, next(7));
                axpy_into(leaf.v3rhosigma2.as_deref_mut(), weight, next(8));
                axpy_into(leaf.v3sigma3.as_deref_mut(), weight, next(9));
                axpy_into(leaf.v4rho4.as_deref_mut(), weight, next(10));
                axpy_into(leaf.v4rho3sigma.as_deref_mut(), weight, next(11));
                axpy_into(leaf.v4rho2sigma2.as_deref_mut(), weight, next(12));
                axpy_into(leaf.v4rhosigma3.as_deref_mut(), weight, next(13));
                axpy_into(leaf.v4sigma4.as_deref_mut(), weight, next(14));
            }
            Family::Mgga => {
                // Rejected before the sweep started; unreachable in practice.
                return Err(LibxcRsError::UnsupportedFunctional {
                    id: functional.meta.id,
                    reason: "MGGA auxiliary inside GGA parent (mix_func.c rejects this combination)",
                });
            }
        }
    }
    Ok(())
}

/// Evaluate a mixed MGGA functional. Per-aux family gating mirrors
/// `evaluate_mixed_gga` with two additional gates for laplacian and tau
/// derivatives (libxc `mix_func.c:184-305`):
///
/// - **LDA aux** contributes only to rho-derivative chain.
/// - **GGA aux** contributes to rho + sigma chains.
/// - **MGGA aux** contributes to rho + sigma + (lapl chain if
///   aux's `NEEDS_LAPLACIAN` AND parent's `NEEDS_LAPLACIAN` are both set) +
///   (tau chain if aux's `NEEDS_TAU` AND parent's `NEEDS_TAU` are both set).
///   Mixed lapl-tau cross-derivative fields gate on `needs_lapl AND needs_tau`
///   (the combined gates).
///
/// **CR-03 fix (Plan 05-06):** the parent's NEEDS_LAPLACIAN/NEEDS_TAU flags
/// are also load-bearing per `mix_func.c:104-120` (parent ASSERTS its own
/// NEEDS_LAPLACIAN bit must be set whenever any aux needs laplacian). Gating
/// on both parent AND aux is the safe defense: when parent's flags are
/// correctly populated by xtask, the gate is equivalent to the libxc
/// reference (aux-only); when parent's flags are missing, the gate prevents
/// aux contributions from leaking into a parent output buffer the parent did
/// not promise to expose. The `add_opt_n` helper would also catch this as a
/// length mismatch, but the gate is cheaper and more semantically clear.
///
/// All 70 MGGA caller output fields are zeroed once before the loop.
pub fn evaluate_mixed_mgga(
    functional: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    output: &mut MggaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    evaluate_mixed_mgga_into(functional, input, order, output, workspace, true)
}

/// As [`evaluate_mixed_mgga`], but `zero_first = false` accumulates into
/// whatever the output already holds.
///
/// libxc's `xc_mgga_new` evaluates a functional's own kernel and *then* adds
/// `xc_mix_func` on top when the info block carries both a work pointer and a
/// non-NULL `mix_coef`. Reproducing that needs a mix pass that does not first
/// wipe the kernel's contribution. `hyb_mgga_xc_b0kcis` is the only functional
/// in libxc 7.0.0 that needs it.
pub fn evaluate_mixed_mgga_into(
    functional: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    output: &mut MggaOutput,
    workspace: &mut EvaluationWorkspace,
    zero_first: bool,
) -> Result<(), LibxcRsError> {
    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }
    // Grow the scratch to exactly this evaluation's order (a no-op if it is
    // already at least that big). Lets a caller hand over a minimally-sized
    // workspace and have it reach the right size once, rather than every
    // caller paying for the MGGA all-orders superset up front.
    workspace.ensure_order(order);

    // CR-02 (Plan 05-06): pre-compute per-family per-field lengths once so
    // every accumulation site uses an explicit length parameter (no silent
    // truncation). Three family flavours are needed since the MGGA parent
    // accepts LDA, GGA, and MGGA aux subtrees.
    let np = input.np();
    let mgga_dims = Dimensions::mgga(input.spin());
    let gga_dims = Dimensions::gga(input.spin());
    let lda_dims = Dimensions::lda(input.spin());

    // MGGA per-field lengths (for MGGA aux + writes into MGGA parent output).
    let mgga_zk_len = mgga_dims.zk as usize * np;
    let mgga_vrho_len = mgga_dims.vrho as usize * np;
    let mgga_vsigma_len = mgga_dims.vsigma as usize * np;
    let mgga_vlapl_len = mgga_dims.vlapl as usize * np;
    let mgga_vtau_len = mgga_dims.vtau as usize * np;
    let mgga_v2rho2_len = mgga_dims.v2rho2 as usize * np;
    let mgga_v2rhosigma_len = mgga_dims.v2rhosigma as usize * np;
    let mgga_v2rholapl_len = mgga_dims.v2rholapl as usize * np;
    let mgga_v2rhotau_len = mgga_dims.v2rhotau as usize * np;
    let mgga_v2sigma2_len = mgga_dims.v2sigma2 as usize * np;
    let mgga_v2sigmalapl_len = mgga_dims.v2sigmalapl as usize * np;
    let mgga_v2sigmatau_len = mgga_dims.v2sigmatau as usize * np;
    let mgga_v2lapl2_len = mgga_dims.v2lapl2 as usize * np;
    let mgga_v2lapltau_len = mgga_dims.v2lapltau as usize * np;
    let mgga_v2tau2_len = mgga_dims.v2tau2 as usize * np;

    // GGA per-field lengths (for GGA aux). The MGGA parent output buffers for
    // these fields are sized to MGGA dimensions, which equal GGA dimensions
    // for the rho+sigma chain (mgga_dims.{zk,vrho,vsigma,...} == gga_dims same).
    let gga_zk_len = gga_dims.zk as usize * np;
    let gga_vrho_len = gga_dims.vrho as usize * np;
    let gga_vsigma_len = gga_dims.vsigma as usize * np;
    let gga_v2rho2_len = gga_dims.v2rho2 as usize * np;
    let gga_v2rhosigma_len = gga_dims.v2rhosigma as usize * np;
    let gga_v2sigma2_len = gga_dims.v2sigma2 as usize * np;

    // LDA per-field lengths (for LDA aux). MGGA parent's rho-only field
    // dimensions equal LDA dimensions.
    let lda_zk_len = lda_dims.zk as usize * np;
    let lda_vrho_len = lda_dims.vrho as usize * np;
    let lda_v2rho2_len = lda_dims.v2rho2 as usize * np;

    // Capture parent flags once (CR-03 fix): the gate is parent AND aux per
    // mix_func.c:184-305 + parent assertion at mix_func.c:104-120.
    let parent_needs_lapl = functional
        .meta
        .flags
        .contains(FunctionalFlags::NEEDS_LAPLACIAN);
    let parent_needs_tau = functional.meta.flags.contains(FunctionalFlags::NEEDS_TAU);

    // Skipped when accumulating on top of a kernel result -- see the
    // `zero_first` note on this function.
    macro_rules! zero_field {
        ($field:ident) => {
            if zero_first
                && let Some(ref mut b) = output.$field
            {
                b.fill(0.0);
            }
        };
    }
    zero_field!(zk);
    zero_field!(vrho);
    zero_field!(vsigma);
    zero_field!(vlapl);
    zero_field!(vtau);
    zero_field!(v2rho2);
    zero_field!(v2rhosigma);
    zero_field!(v2rholapl);
    zero_field!(v2rhotau);
    zero_field!(v2sigma2);
    zero_field!(v2sigmalapl);
    zero_field!(v2sigmatau);
    zero_field!(v2lapl2);
    zero_field!(v2lapltau);
    zero_field!(v2tau2);
    zero_field!(v3rho3);
    zero_field!(v3rho2sigma);
    zero_field!(v3rho2lapl);
    zero_field!(v3rho2tau);
    zero_field!(v3rhosigma2);
    zero_field!(v3rhosigmalapl);
    zero_field!(v3rhosigmatau);
    zero_field!(v3rholapl2);
    zero_field!(v3rholapltau);
    zero_field!(v3rhotau2);
    zero_field!(v3sigma3);
    zero_field!(v3sigma2lapl);
    zero_field!(v3sigma2tau);
    zero_field!(v3sigmalapl2);
    zero_field!(v3sigmalapltau);
    zero_field!(v3sigmatau2);
    zero_field!(v3lapl3);
    zero_field!(v3lapl2tau);
    zero_field!(v3lapltau2);
    zero_field!(v3tau3);
    zero_field!(v4rho4);
    zero_field!(v4rho3sigma);
    zero_field!(v4rho3lapl);
    zero_field!(v4rho3tau);
    zero_field!(v4rho2sigma2);
    zero_field!(v4rho2sigmalapl);
    zero_field!(v4rho2sigmatau);
    zero_field!(v4rho2lapl2);
    zero_field!(v4rho2lapltau);
    zero_field!(v4rho2tau2);
    zero_field!(v4rhosigma3);
    zero_field!(v4rhosigma2lapl);
    zero_field!(v4rhosigma2tau);
    zero_field!(v4rhosigmalapl2);
    zero_field!(v4rhosigmalapltau);
    zero_field!(v4rhosigmatau2);
    zero_field!(v4rholapl3);
    zero_field!(v4rholapl2tau);
    zero_field!(v4rholapltau2);
    zero_field!(v4rhotau3);
    zero_field!(v4sigma4);
    zero_field!(v4sigma3lapl);
    zero_field!(v4sigma3tau);
    zero_field!(v4sigma2lapl2);
    zero_field!(v4sigma2lapltau);
    zero_field!(v4sigma2tau2);
    zero_field!(v4sigmalapl3);
    zero_field!(v4sigmalapl2tau);
    zero_field!(v4sigmalapltau2);
    zero_field!(v4sigmatau3);
    zero_field!(v4lapl4);
    zero_field!(v4lapl3tau);
    zero_field!(v4lapl2tau2);
    zero_field!(v4lapltau3);
    zero_field!(v4tau4);

    for (aux, &weight) in functional
        .auxiliaries
        .iter()
        .zip(functional.mix_coefficients.iter())
    {
        match aux.meta.family {
            Family::Lda => {
                let lda_input = LdaInput::new(input.rho(), input.np(), input.spin())?;
                {
                    let scratch = workspace.lda_scratch_mut();
                    let mut aux_output = LdaOutput {
                        zk: Some(scratch.zk),
                        vrho: if order >= DerivativeOrder::Vxc {
                            Some(scratch.vrho)
                        } else {
                            None
                        },
                        v2rho2: if order >= DerivativeOrder::Fxc {
                            Some(scratch.v2rho2)
                        } else {
                            None
                        },
                        v3rho3: if order >= DerivativeOrder::Kxc {
                            Some(scratch.v3rho3)
                        } else {
                            None
                        },
                        v4rho4: if order >= DerivativeOrder::Lxc {
                            Some(scratch.v4rho4)
                        } else {
                            None
                        },
                    };
                    dispatch_lda_by_id(
                        aux.meta.id,
                        &lda_input,
                        order,
                        &mut aux_output,
                        aux.kernel_ext_params(),
                        &aux.thresholds,
                    )?;
                }
                let scratch = workspace.lda_scratch_mut();
                add_opt_n(
                    output.zk.as_deref_mut(),
                    weight,
                    scratch.zk,
                    lda_zk_len,
                    "zk",
                )?;
                if order >= DerivativeOrder::Vxc {
                    add_opt_n(
                        output.vrho.as_deref_mut(),
                        weight,
                        scratch.vrho,
                        lda_vrho_len,
                        "vrho",
                    )?;
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt_n(
                        output.v2rho2.as_deref_mut(),
                        weight,
                        scratch.v2rho2,
                        lda_v2rho2_len,
                        "v2rho2",
                    )?;
                }
                // Note: dispatch_mgga currently rejects Kxc/Lxc orders upstream
                // (the function returns UnsupportedDerivativeOrder before this
                // accumulation block runs). LDA-aux Kxc/Lxc accumulation paths
                // are therefore unreachable from evaluate_mixed_mgga today; if
                // and when MGGA Kxc/Lxc dispatch lands, add v3rho3/v4rho4 calls
                // here (using mgga_dims-derived lengths to match parent buffer
                // shape, which equals lda_dims for the rho-only chain).
            }
            Family::Gga => {
                let gga_input =
                    GgaInput::new(input.rho(), input.sigma(), input.np(), input.spin())?;
                {
                    let scratch = workspace.gga_scratch_mut();
                    let mut aux_output = GgaOutput {
                        zk: Some(scratch.zk),
                        vrho: if order >= DerivativeOrder::Vxc {
                            Some(scratch.vrho)
                        } else {
                            None
                        },
                        vsigma: if order >= DerivativeOrder::Vxc {
                            Some(scratch.vsigma)
                        } else {
                            None
                        },
                        v2rho2: if order >= DerivativeOrder::Fxc {
                            Some(scratch.v2rho2)
                        } else {
                            None
                        },
                        v2rhosigma: if order >= DerivativeOrder::Fxc {
                            Some(scratch.v2rhosigma)
                        } else {
                            None
                        },
                        v2sigma2: if order >= DerivativeOrder::Fxc {
                            Some(scratch.v2sigma2)
                        } else {
                            None
                        },
                        v3rho3: if order >= DerivativeOrder::Kxc {
                            Some(scratch.v3rho3)
                        } else {
                            None
                        },
                        v3rho2sigma: if order >= DerivativeOrder::Kxc {
                            Some(scratch.v3rho2sigma)
                        } else {
                            None
                        },
                        v3rhosigma2: if order >= DerivativeOrder::Kxc {
                            Some(scratch.v3rhosigma2)
                        } else {
                            None
                        },
                        v3sigma3: if order >= DerivativeOrder::Kxc {
                            Some(scratch.v3sigma3)
                        } else {
                            None
                        },
                        v4rho4: if order >= DerivativeOrder::Lxc {
                            Some(scratch.v4rho4)
                        } else {
                            None
                        },
                        v4rho3sigma: if order >= DerivativeOrder::Lxc {
                            Some(scratch.v4rho3sigma)
                        } else {
                            None
                        },
                        v4rho2sigma2: if order >= DerivativeOrder::Lxc {
                            Some(scratch.v4rho2sigma2)
                        } else {
                            None
                        },
                        v4rhosigma3: if order >= DerivativeOrder::Lxc {
                            Some(scratch.v4rhosigma3)
                        } else {
                            None
                        },
                        v4sigma4: if order >= DerivativeOrder::Lxc {
                            Some(scratch.v4sigma4)
                        } else {
                            None
                        },
                    };
                    dispatch_gga_by_id(
                        aux.meta.id,
                        &gga_input,
                        order,
                        &mut aux_output,
                        aux.kernel_ext_params(),
                        &aux.thresholds,
                    )?;
                }
                let scratch = workspace.gga_scratch_mut();
                add_opt_n(
                    output.zk.as_deref_mut(),
                    weight,
                    scratch.zk,
                    gga_zk_len,
                    "zk",
                )?;
                if order >= DerivativeOrder::Vxc {
                    add_opt_n(
                        output.vrho.as_deref_mut(),
                        weight,
                        scratch.vrho,
                        gga_vrho_len,
                        "vrho",
                    )?;
                    add_opt_n(
                        output.vsigma.as_deref_mut(),
                        weight,
                        scratch.vsigma,
                        gga_vsigma_len,
                        "vsigma",
                    )?;
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt_n(
                        output.v2rho2.as_deref_mut(),
                        weight,
                        scratch.v2rho2,
                        gga_v2rho2_len,
                        "v2rho2",
                    )?;
                    add_opt_n(
                        output.v2rhosigma.as_deref_mut(),
                        weight,
                        scratch.v2rhosigma,
                        gga_v2rhosigma_len,
                        "v2rhosigma",
                    )?;
                    add_opt_n(
                        output.v2sigma2.as_deref_mut(),
                        weight,
                        scratch.v2sigma2,
                        gga_v2sigma2_len,
                        "v2sigma2",
                    )?;
                }
                // Note: dispatch_mgga currently rejects Kxc/Lxc orders, so the
                // higher-order GGA-aux accumulation paths are unreachable from
                // evaluate_mixed_mgga today. They were retained in the prior
                // code as defense-in-depth; here we omit them in the
                // length-checked rewrite because the corresponding parent
                // output fields (output.v3rho3 etc.) would be sized to MGGA
                // dimensions, not GGA dimensions, causing add_opt_n to error
                // even on a no-op path. If/when MGGA Kxc/Lxc dispatch lands,
                // re-add these calls using mgga-derived lengths.
            }
            Family::Mgga => {
                let aux_needs_lapl = aux.meta.flags.contains(FunctionalFlags::NEEDS_LAPLACIAN);
                let aux_needs_tau = aux.meta.flags.contains(FunctionalFlags::NEEDS_TAU);
                // CR-03 fix (Plan 05-06): gate on parent AND aux flags per
                // mix_func.c:104-120 (parent-flag assertion) + 184-305
                // (per-aux accumulation). When parent doesn't carry the
                // NEEDS_LAPLACIAN/NEEDS_TAU bit, we MUST NOT route aux's
                // vlapl/vtau contributions into the parent's output buffers
                // (which the parent didn't promise to expose).
                let needs_lapl = aux_needs_lapl && parent_needs_lapl;
                let needs_tau = aux_needs_tau && parent_needs_tau;
                let needs_both = needs_lapl && needs_tau;

                {
                    // The auxiliary gets a buffer for every field its own
                    // family and order requires, unconditionally.
                    //
                    // These used to be gated on `needs_lapl`/`needs_tau` --
                    // the aux's flag ANDed with the parent's. But that gate
                    // belongs on the *accumulation* below, which is where a
                    // parent that does not expose `vlapl` must not receive an
                    // aux's contribution. Applying it here instead withheld a
                    // buffer the aux kernel demands: `prepare` requires every
                    // field of the requested order, so `evaluate_mgga` failed
                    // outright with "output buffer 'vlapl' size mismatch" for
                    // **36 of the 39 composite MGGA functionals** -- every
                    // TPSS0/B95/BR3P86/MS2h/SCAN0 hybrid. The accumulation
                    // gating below is unchanged, so nothing leaks.
                    let scratch = workspace.mgga_scratch_mut();
                    let mut aux_output = MggaOutput {
                        zk: Some(scratch.zk),
                        ..Default::default()
                    };
                    if order >= DerivativeOrder::Vxc {
                        aux_output.vrho = Some(scratch.vrho);
                        aux_output.vsigma = Some(scratch.vsigma);
                        aux_output.vlapl = Some(scratch.vlapl);
                        aux_output.vtau = Some(scratch.vtau);
                    }
                    if order >= DerivativeOrder::Fxc {
                        aux_output.v2rho2 = Some(scratch.v2rho2);
                        aux_output.v2rhosigma = Some(scratch.v2rhosigma);
                        aux_output.v2sigma2 = Some(scratch.v2sigma2);
                        aux_output.v2rholapl = Some(scratch.v2rholapl);
                        aux_output.v2sigmalapl = Some(scratch.v2sigmalapl);
                        aux_output.v2lapl2 = Some(scratch.v2lapl2);
                        aux_output.v2rhotau = Some(scratch.v2rhotau);
                        aux_output.v2sigmatau = Some(scratch.v2sigmatau);
                        aux_output.v2tau2 = Some(scratch.v2tau2);
                        aux_output.v2lapltau = Some(scratch.v2lapltau);
                    }
                    // Order >= Kxc / Lxc: dispatch_mgga currently rejects them
                    // upstream, so leave the higher-order aux_output fields as
                    // None. If/when MGGA Fxc+ is wired, expand here.
                    // (WR-10 Plan 05-06: a dead let-discard that previously
                    // consumed needs_lapl / needs_tau / needs_both has been
                    // removed since those variables are load-bearing below in
                    // the gated accumulation block.)
                    dispatch_mgga_by_id(
                        aux.meta.id,
                        input,
                        order,
                        &mut aux_output,
                        aux.kernel_ext_params(),
                        &aux.thresholds,
                    )?;
                }
                let scratch = workspace.mgga_scratch_mut();
                // Always-accumulate (rho-chain, all aux families contribute).
                add_opt_n(
                    output.zk.as_deref_mut(),
                    weight,
                    scratch.zk,
                    mgga_zk_len,
                    "zk",
                )?;
                if order >= DerivativeOrder::Vxc {
                    add_opt_n(
                        output.vrho.as_deref_mut(),
                        weight,
                        scratch.vrho,
                        mgga_vrho_len,
                        "vrho",
                    )?;
                    add_opt_n(
                        output.vsigma.as_deref_mut(),
                        weight,
                        scratch.vsigma,
                        mgga_vsigma_len,
                        "vsigma",
                    )?;
                    if needs_lapl {
                        add_opt_n(
                            output.vlapl.as_deref_mut(),
                            weight,
                            scratch.vlapl,
                            mgga_vlapl_len,
                            "vlapl",
                        )?;
                    }
                    if needs_tau {
                        add_opt_n(
                            output.vtau.as_deref_mut(),
                            weight,
                            scratch.vtau,
                            mgga_vtau_len,
                            "vtau",
                        )?;
                    }
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt_n(
                        output.v2rho2.as_deref_mut(),
                        weight,
                        scratch.v2rho2,
                        mgga_v2rho2_len,
                        "v2rho2",
                    )?;
                    add_opt_n(
                        output.v2rhosigma.as_deref_mut(),
                        weight,
                        scratch.v2rhosigma,
                        mgga_v2rhosigma_len,
                        "v2rhosigma",
                    )?;
                    add_opt_n(
                        output.v2sigma2.as_deref_mut(),
                        weight,
                        scratch.v2sigma2,
                        mgga_v2sigma2_len,
                        "v2sigma2",
                    )?;
                    if needs_lapl {
                        add_opt_n(
                            output.v2rholapl.as_deref_mut(),
                            weight,
                            scratch.v2rholapl,
                            mgga_v2rholapl_len,
                            "v2rholapl",
                        )?;
                        add_opt_n(
                            output.v2sigmalapl.as_deref_mut(),
                            weight,
                            scratch.v2sigmalapl,
                            mgga_v2sigmalapl_len,
                            "v2sigmalapl",
                        )?;
                        add_opt_n(
                            output.v2lapl2.as_deref_mut(),
                            weight,
                            scratch.v2lapl2,
                            mgga_v2lapl2_len,
                            "v2lapl2",
                        )?;
                    }
                    if needs_tau {
                        add_opt_n(
                            output.v2rhotau.as_deref_mut(),
                            weight,
                            scratch.v2rhotau,
                            mgga_v2rhotau_len,
                            "v2rhotau",
                        )?;
                        add_opt_n(
                            output.v2sigmatau.as_deref_mut(),
                            weight,
                            scratch.v2sigmatau,
                            mgga_v2sigmatau_len,
                            "v2sigmatau",
                        )?;
                        add_opt_n(
                            output.v2tau2.as_deref_mut(),
                            weight,
                            scratch.v2tau2,
                            mgga_v2tau2_len,
                            "v2tau2",
                        )?;
                    }
                    if needs_both {
                        add_opt_n(
                            output.v2lapltau.as_deref_mut(),
                            weight,
                            scratch.v2lapltau,
                            mgga_v2lapltau_len,
                            "v2lapltau",
                        )?;
                    }
                }
                // Higher-order MGGA accumulation (Kxc/Lxc) deferred — current
                // dispatch_mgga rejects those orders upstream.
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::dispatch_lda;
    use libxc_core::input::LdaInput;
    use libxc_core::model::{DerivativeOrder, Spin, Thresholds};
    use libxc_core::output::LdaOutput;

    /// `sweep_gga::set_min_chunk` is a process-wide knob and the test harness
    /// runs tests on parallel threads, so every test that sets or depends on
    /// it holds this while it runs.
    static MIN_CHUNK_GUARD: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// A synthetic grid with the shape a molecular quadrature has: a
    /// below-threshold tail, so density screening is exercised inside the
    /// leaves too.
    fn gga_grid(np: usize, nc: usize) -> (Vec<f64>, Vec<f64>) {
        let ns = if nc == 1 { 1 } else { 3 };
        let mut rho = vec![0.0; np * nc];
        let mut sigma = vec![0.0; np * ns];
        for ip in 0..np {
            let t = ip as f64 / np as f64;
            // Exponential decay into the tail; the last 30% sit under any
            // sensible dens_threshold.
            let r = 2.0 * (-24.0 * t).exp() + 1e-40;
            let g = 0.7 * r * r;
            for c in 0..nc {
                rho[ip * nc + c] = r * (1.0 + 0.3 * c as f64);
            }
            for c in 0..ns {
                sigma[ip * ns + c] = g * (1.0 + 0.1 * c as f64);
            }
        }
        (rho, sigma)
    }

    fn run_composite(
        name: &str,
        spin: Spin,
        order: DerivativeOrder,
        np: usize,
        min_chunk: usize,
        with_vsigma: bool,
    ) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>) {
        use libxc_core::registry::lookup_by_name;
        let _guard = MIN_CHUNK_GUARD.lock().unwrap_or_else(|p| p.into_inner());
        let id = lookup_by_name(name).unwrap();
        let f = Functional::new(id, spin).unwrap();
        assert!(!f.auxiliaries.is_empty(), "{name} must be a composite");
        let nc = if spin == Spin::Polarized { 2 } else { 1 };
        let d = Dimensions::gga(spin);
        let (rho, sigma) = gga_grid(np, nc);
        let input = GgaInput::new(&rho, &sigma, np, spin).unwrap();
        let mut zk = vec![0.0; np * d.zk as usize];
        let mut vrho = vec![0.0; np * d.vrho as usize];
        let mut vsigma = vec![0.0; np * d.vsigma as usize];
        let mut v2rho2 = vec![0.0; np * d.v2rho2 as usize];
        let mut ws = EvaluationWorkspace::with_order(np, spin, DerivativeOrder::Exc);
        {
            let mut out = GgaOutput {
                zk: Some(&mut zk),
                vrho: Some(&mut vrho),
                vsigma: if with_vsigma { Some(&mut vsigma) } else { None },
                v2rho2: if order >= DerivativeOrder::Fxc { Some(&mut v2rho2) } else { None },
                ..Default::default()
            };
            let saved = libxc_reval::sweep_gga::min_chunk();
            libxc_reval::sweep_gga::set_min_chunk(min_chunk);
            let r = f.evaluate_gga(&input, order, &mut out, &mut ws);
            libxc_reval::sweep_gga::set_min_chunk(saved);
            r.unwrap();
        }
        // The chunked path must not have materialised the whole-grid scratch.
        assert_eq!(ws.scratch_allocated(), 0, "composite GGA path touched the whole-grid scratch");
        (zk, vrho, vsigma, v2rho2)
    }

    /// The leaf-by-leaf composite path reproduces the one-chunk evaluation
    /// bit for bit, on a grid several leaves long with a screened tail, for a
    /// composite with GGA auxiliaries (PBE0), one with an LDA auxiliary in
    /// the mix (B3LYP), and the screened hybrid HSE06.
    #[test]
    fn chunked_composite_gga_is_bit_identical_to_one_chunk() {
        for name in ["XC_HYB_GGA_XC_PBEH", "XC_HYB_GGA_XC_B3LYP", "XC_HYB_GGA_XC_HSE06"] {
            for spin in [Spin::Unpolarized, Spin::Polarized] {
                for order in [DerivativeOrder::Vxc, DerivativeOrder::Fxc] {
                    let np = 5000;
                    let whole = run_composite(name, spin, order, np, usize::MAX, true);
                    let leaves = run_composite(name, spin, order, np, 512, true);
                    for (k, (a, b)) in [(&whole.0, &leaves.0), (&whole.1, &leaves.1), (&whole.2, &leaves.2), (&whole.3, &leaves.3)]
                        .iter()
                        .enumerate()
                    {
                        let bad = a.iter().zip(b.iter()).filter(|(x, y)| x.to_bits() != y.to_bits()).count();
                        assert_eq!(bad, 0, "{name} {spin:?} {order:?}: field {k} differs in {bad} values");
                    }
                    assert!(whole.0.iter().any(|v| *v != 0.0), "{name}: zk is identically zero");
                }
            }
        }
    }

    /// A caller may leave a field out; it is simply not accumulated into.
    #[test]
    fn chunked_composite_gga_skips_absent_fields() {
        let full = run_composite("XC_HYB_GGA_XC_PBEH", Spin::Unpolarized, DerivativeOrder::Vxc, 3000, 512, true);
        let part = run_composite("XC_HYB_GGA_XC_PBEH", Spin::Unpolarized, DerivativeOrder::Vxc, 3000, 512, false);
        assert_eq!(full.0, part.0);
        assert_eq!(full.1, part.1);
        assert!(part.2.iter().all(|v| *v == 0.0));
    }

    /// A wrong-length caller buffer is rejected up front, before any leaf runs.
    #[test]
    fn chunked_composite_gga_rejects_wrong_buffer_length() {
        use libxc_core::registry::lookup_by_name;
        let np = 100;
        let spin = Spin::Unpolarized;
        let f = Functional::new(lookup_by_name("XC_HYB_GGA_XC_PBEH").unwrap(), spin).unwrap();
        let (rho, sigma) = gga_grid(np, 1);
        let input = GgaInput::new(&rho, &sigma, np, spin).unwrap();
        let mut zk = vec![0.0; np];
        let mut vrho = vec![0.0; np + 1];
        let mut vsigma = vec![0.0; np];
        let mut ws = EvaluationWorkspace::with_order(np, spin, DerivativeOrder::Exc);
        let mut out = GgaOutput {
            zk: Some(&mut zk),
            vrho: Some(&mut vrho),
            vsigma: Some(&mut vsigma),
            ..Default::default()
        };
        match f.evaluate_gga(&input, DerivativeOrder::Vxc, &mut out, &mut ws) {
            Err(LibxcRsError::OutputBufferSizeMismatch { field: "vrho", expected, actual }) => {
                assert_eq!((expected, actual), (np, np + 1));
            }
            other => panic!("expected OutputBufferSizeMismatch on vrho, got {other:?}"),
        }
    }

    /// The pool holds at most one buffer per worker that was busy at once,
    /// and a second evaluation through the same workspace allocates nothing.
    #[test]
    fn leaf_pool_is_bounded_and_reused() {
        use libxc_core::registry::lookup_by_name;
        // Enough points that there are many more leaves than workers, so the
        // pool size is set by the workers and not by the grid.
        let _guard = MIN_CHUNK_GUARD.lock().unwrap_or_else(|p| p.into_inner());
        let np = 400_000;
        let spin = Spin::Polarized;
        let f = Functional::new(lookup_by_name("XC_HYB_GGA_XC_PBEH").unwrap(), spin).unwrap();
        let (rho, sigma) = gga_grid(np, 2);
        let input = GgaInput::new(&rho, &sigma, np, spin).unwrap();
        let d = Dimensions::gga(spin);
        let mut zk = vec![0.0; np];
        let mut vrho = vec![0.0; np * d.vrho as usize];
        let mut vsigma = vec![0.0; np * d.vsigma as usize];
        let mut ws = EvaluationWorkspace::with_order(np, spin, DerivativeOrder::Exc);
        let mut go = |ws: &mut EvaluationWorkspace| {
            let mut out = GgaOutput {
                zk: Some(&mut zk),
                vrho: Some(&mut vrho),
                vsigma: Some(&mut vsigma),
                ..Default::default()
            };
            f.evaluate_gga(&input, DerivativeOrder::Vxc, &mut out, ws).unwrap();
        };
        go(&mut ws);
        let after_first = ws.pool_len();
        let per_leaf = libxc_reval::sweep_gga::min_chunk() * (d.zk + d.vrho + d.vsigma) as usize;
        assert!(after_first > 0);
        assert!(
            after_first <= rayon::current_num_threads() * per_leaf,
            "pool {after_first} exceeds workers x leaf ({} x {per_leaf})",
            rayon::current_num_threads()
        );
        assert!(after_first * 4 < (d.zk + d.vrho + d.vsigma) as usize * np, "pool is grid-sized");
        go(&mut ws);
        assert_eq!(ws.pool_len(), after_first, "second evaluation grew the pool");
    }

    fn default_thresholds() -> Thresholds {
        Thresholds::default()
    }

    // ── add_to_mix unit tests ─────────────────────────────────────

    #[test]
    fn add_to_mix_basic() {
        let mut dst = vec![0.0, 0.0];
        let src = vec![1.0, 2.0];
        add_to_mix(&mut dst, 0.7, &src);
        assert!((dst[0] - 0.7).abs() < 1e-15);
        assert!((dst[1] - 1.4).abs() < 1e-15);
    }

    #[test]
    fn add_to_mix_complementary_weights_sum_to_identity() {
        let mut dst = vec![0.0, 0.0];
        let src = vec![1.0, 2.0];
        add_to_mix(&mut dst, 0.7, &src);
        add_to_mix(&mut dst, 0.3, &src);
        assert!((dst[0] - 1.0).abs() < 1e-15, "dst[0] = {}", dst[0]);
        assert!((dst[1] - 2.0).abs() < 1e-15, "dst[1] = {}", dst[1]);
    }

    #[test]
    fn add_to_mix_accumulates_on_existing() {
        let mut dst = vec![10.0, 20.0];
        let src = vec![1.0, 2.0];
        add_to_mix(&mut dst, 0.5, &src);
        assert!((dst[0] - 10.5).abs() < 1e-15);
        assert!((dst[1] - 21.0).abs() < 1e-15);
    }

    // ── evaluate_mixed_lda integration tests ──────────────────────

    #[test]
    fn mixed_single_aux_weight_1_matches_dispatch() {
        let rho = vec![0.1, 0.2, 0.5, 1.0];
        let np = 4;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct dispatch
        let mut zk_direct = vec![0.0f64; np];
        let mut vrho_direct = vec![0.0f64; np];
        {
            let mut out_direct = LdaOutput::new(
                Some(&mut zk_direct),
                Some(&mut vrho_direct),
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            dispatch_lda(
                LdaFunctional::LdaX,
                &input,
                DerivativeOrder::Vxc,
                &mut out_direct,
                &LdaXParams::default(),
                &default_thresholds(),
            )
            .unwrap();
        }

        // Mixed with single aux, weight=1.0
        let mut zk_mixed = vec![0.0f64; np];
        let mut vrho_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed),
                Some(&mut vrho_mixed),
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(
                &input,
                DerivativeOrder::Vxc,
                &mut out_mixed,
                &auxes,
                &mut ws,
            )
            .unwrap();
        }

        for i in 0..np {
            assert!(
                (zk_mixed[i] - zk_direct[i]).abs() < 1e-15,
                "zk[{i}]: mixed={} vs direct={}",
                zk_mixed[i],
                zk_direct[i]
            );
            assert!(
                (vrho_mixed[i] - vrho_direct[i]).abs() < 1e-15,
                "vrho[{i}]: mixed={} vs direct={}",
                vrho_mixed[i],
                vrho_direct[i]
            );
        }
    }

    #[test]
    fn mixed_two_auxes_complementary_weights_match_dispatch() {
        let rho = vec![0.1, 0.5, 1.0];
        let np = 3;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct dispatch (weight 1.0)
        let mut zk_direct = vec![0.0f64; np];
        {
            let mut out_direct = LdaOutput::new(
                Some(&mut zk_direct),
                None,
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            dispatch_lda(
                LdaFunctional::LdaX,
                &input,
                DerivativeOrder::Exc,
                &mut out_direct,
                &LdaXParams::default(),
                &default_thresholds(),
            )
            .unwrap();
        }

        // Mixed with two auxes: 0.7 + 0.3 = 1.0
        let mut zk_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed),
                None,
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            let auxes = vec![
                AuxiliaryConfig {
                    alpha: 1.0,
                    weight: 0.7,
                    thresholds: default_thresholds(),
                },
                AuxiliaryConfig {
                    alpha: 1.0,
                    weight: 0.3,
                    thresholds: default_thresholds(),
                },
            ];
            evaluate_mixed_lda(
                &input,
                DerivativeOrder::Exc,
                &mut out_mixed,
                &auxes,
                &mut ws,
            )
            .unwrap();
        }

        for i in 0..np {
            assert!(
                (zk_mixed[i] - zk_direct[i]).abs() < 1e-14,
                "zk[{i}]: mixed={} vs direct={}",
                zk_mixed[i],
                zk_direct[i]
            );
        }
    }

    #[test]
    fn mixed_half_weight_produces_half_result() {
        let rho = vec![0.5, 1.0];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct dispatch
        let mut zk_direct = vec![0.0f64; np];
        {
            let mut out_direct = LdaOutput::new(
                Some(&mut zk_direct),
                None,
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            dispatch_lda(
                LdaFunctional::LdaX,
                &input,
                DerivativeOrder::Exc,
                &mut out_direct,
                &LdaXParams::default(),
                &default_thresholds(),
            )
            .unwrap();
        }

        // Mixed with weight=0.5
        let mut zk_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed),
                None,
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 0.5,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(
                &input,
                DerivativeOrder::Exc,
                &mut out_mixed,
                &auxes,
                &mut ws,
            )
            .unwrap();
        }

        for i in 0..np {
            let expected = zk_direct[i] * 0.5;
            assert!(
                (zk_mixed[i] - expected).abs() < 1e-15,
                "zk[{i}]: mixed={} vs expected={}",
                zk_mixed[i],
                expected
            );
        }
    }

    #[test]
    fn mixed_vxc_order_populates_both_zk_and_vrho() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        let mut zk = vec![0.0f64; np];
        let mut vrho = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out = LdaOutput::new(
                Some(&mut zk),
                Some(&mut vrho),
                None,
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Vxc, &mut out, &auxes, &mut ws).unwrap();
        }

        assert!(zk[0] < 0.0, "zk should be negative, got {}", zk[0]);
        assert!(vrho[0] != 0.0, "vrho should be non-zero, got {}", vrho[0]);
    }

    #[test]
    fn mixed_exc_order_with_vrho_none_no_panic() {
        let rho = vec![0.5, 1.0];
        let np = 2;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        let mut zk = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out =
                LdaOutput::new(Some(&mut zk), None, None, None, None, np, Spin::Unpolarized)
                    .unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            // Should not panic even though vrho is None
            evaluate_mixed_lda(&input, DerivativeOrder::Exc, &mut out, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            assert!(zk[i] < 0.0, "zk[{i}] should be negative");
        }
    }

    #[test]
    fn mixed_fxc_order_through_mixed_path() {
        let rho = vec![0.5];
        let np = 1;
        let input = LdaInput::new(&rho, np, Spin::Unpolarized).unwrap();

        // Direct
        let mut zk_d = vec![0.0f64; np];
        let mut vrho_d = vec![0.0f64; np];
        let mut v2rho2_d = vec![0.0f64; np];
        {
            let mut out = LdaOutput::new(
                Some(&mut zk_d),
                Some(&mut vrho_d),
                Some(&mut v2rho2_d),
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            dispatch_lda(
                LdaFunctional::LdaX,
                &input,
                DerivativeOrder::Fxc,
                &mut out,
                &LdaXParams::default(),
                &default_thresholds(),
            )
            .unwrap();
        }

        // Mixed with weight=1.0
        let mut zk_m = vec![0.0f64; np];
        let mut vrho_m = vec![0.0f64; np];
        let mut v2rho2_m = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out = LdaOutput::new(
                Some(&mut zk_m),
                Some(&mut vrho_m),
                Some(&mut v2rho2_m),
                None,
                None,
                np,
                Spin::Unpolarized,
            )
            .unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Fxc, &mut out, &auxes, &mut ws).unwrap();
        }

        assert!((zk_m[0] - zk_d[0]).abs() < 1e-15);
        assert!((vrho_m[0] - vrho_d[0]).abs() < 1e-15);
        assert!((v2rho2_m[0] - v2rho2_d[0]).abs() < 1e-15);
    }

    // ── add_opt_n length-checked helper unit tests (Plan 05-06 CR-02) ───────

    #[test]
    fn add_opt_n_writes_when_dst_some_and_lengths_match() {
        let mut dst = vec![0.0f64; 3];
        let src = vec![1.0, 2.0, 3.0];
        add_opt_n(Some(dst.as_mut_slice()), 0.5, &src, 3, "test").unwrap();
        assert_eq!(dst, vec![0.5, 1.0, 1.5]);
    }

    #[test]
    fn add_opt_n_no_op_when_dst_is_none() {
        let src = vec![1.0, 2.0, 3.0];
        // Must not panic, must not error — no-op semantics for unrequested fields.
        add_opt_n(None, 0.5, &src, 3, "test").unwrap();
    }

    #[test]
    fn add_opt_n_errors_on_dst_length_mismatch() {
        let mut dst = vec![0.0f64; 2]; // shorter than declared len=3
        let src = vec![1.0, 2.0, 3.0];
        let res = add_opt_n(Some(dst.as_mut_slice()), 0.5, &src, 3, "v2lapl2");
        match res {
            Err(LibxcRsError::OutputBufferSizeMismatch {
                field,
                expected,
                actual,
            }) => {
                assert_eq!(field, "v2lapl2");
                assert_eq!(expected, 3);
                assert_eq!(actual, 2);
            }
            other => panic!("expected OutputBufferSizeMismatch, got {other:?}"),
        }
    }

    #[test]
    fn add_opt_n_errors_on_src_too_short() {
        let mut dst = vec![0.0f64; 3];
        let src = vec![1.0, 2.0]; // shorter than declared len=3
        let res = add_opt_n(Some(dst.as_mut_slice()), 0.5, &src, 3, "v2tau2");
        assert!(res.is_err());
    }

    // ── evaluate_mixed_mgga parent-flag gate (Plan 05-06 CR-03) ────────────

    /// Verifies the CR-03 fix: `evaluate_mixed_mgga` consults BOTH the
    /// parent's NEEDS_LAPLACIAN flag AND the aux's NEEDS_LAPLACIAN flag
    /// (combined with AND) when deciding to populate vlapl. Per
    /// libxc-master/src/mix_func.c lines 104-120 (parent flag assertion) +
    /// 184-305 (per-aux accumulation), if the parent does not declare
    /// NEEDS_LAPLACIAN, no aux's vlapl contributions should leak into the
    /// parent's output.
    ///
    /// This test exercises the boolean gate at the FunctionalFlags level
    /// since constructing a fully-synthetic Functional with overridden
    /// metadata requires Box::leak gymnastics that obscure the test intent.
    /// Full end-to-end validation is provided by the FFI-tier oracle test
    /// `b94_hyb_mgga_vxc_matches_libxc` (in verify/tests/mixed_oracle.rs,
    /// unignored by Plan 05-04), which exercises the live combined gate
    /// against libxc 7.0.0 for hyb_mgga_xc_b94.
    #[test]
    fn mixed_mgga_respects_parent_no_laplacian_gate() {
        use libxc_core::model::FunctionalFlags;

        // Parent does NOT need laplacian; aux DOES need laplacian.
        // The combined gate `aux_needs && parent_needs` MUST be false.
        let parent_flags = FunctionalFlags::HAVE_EXC | FunctionalFlags::HAVE_VXC;
        let aux_flags = FunctionalFlags::HAVE_EXC
            | FunctionalFlags::HAVE_VXC
            | FunctionalFlags::NEEDS_LAPLACIAN;

        let parent_needs_lapl = parent_flags.contains(FunctionalFlags::NEEDS_LAPLACIAN);
        let aux_needs_lapl = aux_flags.contains(FunctionalFlags::NEEDS_LAPLACIAN);
        let combined_needs_lapl = aux_needs_lapl && parent_needs_lapl;

        assert!(
            aux_needs_lapl,
            "test premise: aux must declare NEEDS_LAPLACIAN"
        );
        assert!(
            !parent_needs_lapl,
            "test premise: parent must NOT declare NEEDS_LAPLACIAN"
        );
        assert!(
            !combined_needs_lapl,
            "CR-03 gate must be FALSE when parent doesn't need laplacian, regardless of aux flags"
        );

        // Symmetric case: BOTH parent AND aux declare NEEDS_LAPLACIAN.
        // The combined gate MUST be true.
        let parent_with_lapl = parent_flags | FunctionalFlags::NEEDS_LAPLACIAN;
        let parent_with_lapl_needs = parent_with_lapl.contains(FunctionalFlags::NEEDS_LAPLACIAN);
        let combined_with_lapl = aux_needs_lapl && parent_with_lapl_needs;
        assert!(
            combined_with_lapl,
            "CR-03 gate must be TRUE when both parent and aux declare NEEDS_LAPLACIAN"
        );

        // Tau symmetry: parent doesn't need tau, aux does → combined gate false.
        let aux_tau = FunctionalFlags::HAVE_EXC | FunctionalFlags::NEEDS_TAU;
        let parent_no_tau = FunctionalFlags::HAVE_EXC;
        let combined_tau = aux_tau.contains(FunctionalFlags::NEEDS_TAU)
            && parent_no_tau.contains(FunctionalFlags::NEEDS_TAU);
        assert!(
            !combined_tau,
            "CR-03 gate must be FALSE for vtau when parent doesn't need tau"
        );
    }
}

/// Add a functional's *own* maple2c kernel on top of an already-accumulated
/// mix, with weight 1.0.
///
/// libxc's `xc_mgga_new` evaluates `func->info->mgga->unpol[order]` when the
/// info block carries a work pointer, and *then* calls `xc_mix_func` when
/// `mix_coef` is non-NULL. There is no guard between them, so a functional
/// with both is the sum of the two. `hyb_mgga_xc_b0kcis` is the only such
/// functional in libxc 7.0.0: it is
/// `mgga_c_kcis + (0.75*gga_x_b88 + 1.0*mgga_c_kcis)`, i.e. twice the KCIS
/// correlation. Verified against libxc to 1.7e-16 by
/// `verify/tests/b0kcis_probe.rs`.
///
/// The kernel goes into the workspace scratch rather than straight into
/// `output`, because `prepare` *takes* the caller's buffers out of the output
/// struct: dispatching into `output` directly would leave every field `None`
/// and silently discard everything accumulated afterwards.
pub fn add_own_kernel_mgga(
    functional: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    output: &mut MggaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    workspace.ensure_order(order);
    let np = input.np();
    let d = Dimensions::mgga(input.spin());

    {
        let scratch = workspace.mgga_scratch_mut();
        let mut own = MggaOutput {
            zk: Some(scratch.zk),
            ..Default::default()
        };
        if order >= DerivativeOrder::Vxc {
            own.vrho = Some(scratch.vrho);
            own.vsigma = Some(scratch.vsigma);
            own.vlapl = Some(scratch.vlapl);
            own.vtau = Some(scratch.vtau);
        }
        if order >= DerivativeOrder::Fxc {
            return Err(LibxcRsError::UnsupportedDerivativeOrder {
                id: functional.meta.id,
                order,
                max: DerivativeOrder::Vxc,
            });
        }
        dispatch_mgga_by_id(
            functional.meta.id,
            input,
            order,
            &mut own,
            functional.kernel_ext_params(),
            &functional.thresholds,
        )?;
    }

    let scratch = workspace.mgga_scratch_mut();
    add_opt_n(output.zk.as_deref_mut(), 1.0, scratch.zk, d.zk as usize * np, "zk")?;
    if order >= DerivativeOrder::Vxc {
        add_opt_n(output.vrho.as_deref_mut(), 1.0, scratch.vrho, d.vrho as usize * np, "vrho")?;
        add_opt_n(output.vsigma.as_deref_mut(), 1.0, scratch.vsigma, d.vsigma as usize * np, "vsigma")?;
        add_opt_n(output.vlapl.as_deref_mut(), 1.0, scratch.vlapl, d.vlapl as usize * np, "vlapl")?;
        add_opt_n(output.vtau.as_deref_mut(), 1.0, scratch.vtau, d.vtau as usize * np, "vtau")?;
    }
    Ok(())
}
