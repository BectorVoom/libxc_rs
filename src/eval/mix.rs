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

use crate::dims::Dimensions;
use crate::error::LibxcRsError;
use crate::eval::dispatch::dispatch_lda;
use crate::eval::gga_dispatch::dispatch_gga;
use crate::eval::mgga_dispatch::dispatch_mgga;
use crate::eval::workspace::EvaluationWorkspace;
use crate::functional::params_lda::LdaXParams;
use crate::functional::Functional;
use crate::input::{GgaInput, LdaInput, MggaInput};
use crate::model::{
    DerivativeOrder, Family, FunctionalFlags, GgaFunctional, LdaFunctional, MggaFunctional,
    Thresholds,
};
use crate::output::{GgaOutput, LdaOutput, MggaOutput};

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
pub fn add_to_mix(dst: &mut [f64], coeff: f64, src: &[f64]) {
    debug_assert_eq!(dst.len(), src.len(), "add_to_mix: dst and src must have equal length");
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
        // Zero scratch before each auxiliary (T-03-07 mitigation).
        // Note: dispatch_lda also zeros its output buffers on entry.
        // The double-zero is intentional -- this call prevents
        // cross-contamination between the accumulation readback and the
        // next dispatch_lda call, while dispatch_lda's internal zero
        // keeps it self-contained for direct (non-mixed) callers.
        workspace.zero_scratch();

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

/// Helper: in-place weighted accumulation that no-ops when `dst` is `None`.
fn add_opt(dst: Option<&mut [f64]>, coeff: f64, src: &[f64]) {
    if let Some(d) = dst {
        // Source slice is the workspace scratch; if it is longer than the
        // caller's output (different family dimensions), only consume the
        // matching prefix. The caller guarantees same-length per family.
        let n = d.len().min(src.len());
        for i in 0..n {
            d[i] += coeff * src[i];
        }
    }
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
    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }

    let np = input.np();
    let dims = Dimensions::lda(input.spin());

    // Zero caller output once before accumulation.
    if let Some(ref mut buf) = output.zk { buf.fill(0.0); }
    if let Some(ref mut buf) = output.vrho { buf.fill(0.0); }
    if let Some(ref mut buf) = output.v2rho2 { buf.fill(0.0); }
    if let Some(ref mut buf) = output.v3rho3 { buf.fill(0.0); }
    if let Some(ref mut buf) = output.v4rho4 { buf.fill(0.0); }

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
        let lda_fn = LdaFunctional::from_id(aux.meta.id)?;

        workspace.zero_scratch();
        {
            let scratch = workspace.lda_scratch_mut();
            let mut scratch_output = LdaOutput {
                zk: Some(scratch.zk),
                vrho: if order >= DerivativeOrder::Vxc { Some(scratch.vrho) } else { None },
                v2rho2: if order >= DerivativeOrder::Fxc { Some(scratch.v2rho2) } else { None },
                v3rho3: if order >= DerivativeOrder::Kxc { Some(scratch.v3rho3) } else { None },
                v4rho4: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho4) } else { None },
            };
            dispatch_lda(
                lda_fn,
                input,
                order,
                &mut scratch_output,
                &*aux.params,
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
/// Caller output is zeroed once before the accumulation loop.
pub fn evaluate_mixed_gga(
    functional: &Functional,
    input: &GgaInput,
    order: DerivativeOrder,
    output: &mut GgaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }

    // Zero all 15 GGA output fields.
    if let Some(ref mut b) = output.zk { b.fill(0.0); }
    if let Some(ref mut b) = output.vrho { b.fill(0.0); }
    if let Some(ref mut b) = output.vsigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v2rho2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v2rhosigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v2sigma2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v3rho3 { b.fill(0.0); }
    if let Some(ref mut b) = output.v3rho2sigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v3rhosigma2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v3sigma3 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rho4 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rho3sigma { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rho2sigma2 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4rhosigma3 { b.fill(0.0); }
    if let Some(ref mut b) = output.v4sigma4 { b.fill(0.0); }

    for (aux, &weight) in functional
        .auxiliaries
        .iter()
        .zip(functional.mix_coefficients.iter())
    {
        match aux.meta.family {
            Family::Lda => {
                // LDA aux: build LdaInput from the GGA input's rho buffer,
                // dispatch into LDA-shaped scratch, accumulate rho-only
                // into the GGA caller output.
                let lda_fn = LdaFunctional::from_id(aux.meta.id)?;
                let lda_input = LdaInput::new(input.rho(), input.np(), input.spin())?;

                workspace.zero_scratch();
                {
                    let scratch = workspace.lda_scratch_mut();
                    let mut aux_output = LdaOutput {
                        zk: Some(scratch.zk),
                        vrho: if order >= DerivativeOrder::Vxc { Some(scratch.vrho) } else { None },
                        v2rho2: if order >= DerivativeOrder::Fxc { Some(scratch.v2rho2) } else { None },
                        v3rho3: if order >= DerivativeOrder::Kxc { Some(scratch.v3rho3) } else { None },
                        v4rho4: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho4) } else { None },
                    };
                    dispatch_lda(
                        lda_fn,
                        &lda_input,
                        order,
                        &mut aux_output,
                        &*aux.params,
                        &aux.thresholds,
                    )?;
                }
                let scratch = workspace.lda_scratch_mut();
                add_opt(output.zk.as_deref_mut(), weight, scratch.zk);
                if order >= DerivativeOrder::Vxc {
                    add_opt(output.vrho.as_deref_mut(), weight, scratch.vrho);
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt(output.v2rho2.as_deref_mut(), weight, scratch.v2rho2);
                }
                if order >= DerivativeOrder::Kxc {
                    add_opt(output.v3rho3.as_deref_mut(), weight, scratch.v3rho3);
                }
                if order >= DerivativeOrder::Lxc {
                    add_opt(output.v4rho4.as_deref_mut(), weight, scratch.v4rho4);
                }
                // Sigma-derivative fields intentionally skipped — Pitfall 5.
            }
            Family::Gga => {
                let gga_fn = GgaFunctional::from_id(aux.meta.id)?;

                workspace.zero_scratch();
                {
                    let scratch = workspace.gga_scratch_mut();
                    let mut aux_output = GgaOutput {
                        zk: Some(scratch.zk),
                        vrho: if order >= DerivativeOrder::Vxc { Some(scratch.vrho) } else { None },
                        vsigma: if order >= DerivativeOrder::Vxc { Some(scratch.vsigma) } else { None },
                        v2rho2: if order >= DerivativeOrder::Fxc { Some(scratch.v2rho2) } else { None },
                        v2rhosigma: if order >= DerivativeOrder::Fxc { Some(scratch.v2rhosigma) } else { None },
                        v2sigma2: if order >= DerivativeOrder::Fxc { Some(scratch.v2sigma2) } else { None },
                        v3rho3: if order >= DerivativeOrder::Kxc { Some(scratch.v3rho3) } else { None },
                        v3rho2sigma: if order >= DerivativeOrder::Kxc { Some(scratch.v3rho2sigma) } else { None },
                        v3rhosigma2: if order >= DerivativeOrder::Kxc { Some(scratch.v3rhosigma2) } else { None },
                        v3sigma3: if order >= DerivativeOrder::Kxc { Some(scratch.v3sigma3) } else { None },
                        v4rho4: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho4) } else { None },
                        v4rho3sigma: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho3sigma) } else { None },
                        v4rho2sigma2: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho2sigma2) } else { None },
                        v4rhosigma3: if order >= DerivativeOrder::Lxc { Some(scratch.v4rhosigma3) } else { None },
                        v4sigma4: if order >= DerivativeOrder::Lxc { Some(scratch.v4sigma4) } else { None },
                    };
                    dispatch_gga(
                        gga_fn,
                        input,
                        order,
                        &mut aux_output,
                        &*aux.params,
                        &aux.thresholds,
                    )?;
                }
                let scratch = workspace.gga_scratch_mut();
                add_opt(output.zk.as_deref_mut(), weight, scratch.zk);
                if order >= DerivativeOrder::Vxc {
                    add_opt(output.vrho.as_deref_mut(), weight, scratch.vrho);
                    add_opt(output.vsigma.as_deref_mut(), weight, scratch.vsigma);
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt(output.v2rho2.as_deref_mut(), weight, scratch.v2rho2);
                    add_opt(output.v2rhosigma.as_deref_mut(), weight, scratch.v2rhosigma);
                    add_opt(output.v2sigma2.as_deref_mut(), weight, scratch.v2sigma2);
                }
                if order >= DerivativeOrder::Kxc {
                    add_opt(output.v3rho3.as_deref_mut(), weight, scratch.v3rho3);
                    add_opt(output.v3rho2sigma.as_deref_mut(), weight, scratch.v3rho2sigma);
                    add_opt(output.v3rhosigma2.as_deref_mut(), weight, scratch.v3rhosigma2);
                    add_opt(output.v3sigma3.as_deref_mut(), weight, scratch.v3sigma3);
                }
                if order >= DerivativeOrder::Lxc {
                    add_opt(output.v4rho4.as_deref_mut(), weight, scratch.v4rho4);
                    add_opt(output.v4rho3sigma.as_deref_mut(), weight, scratch.v4rho3sigma);
                    add_opt(output.v4rho2sigma2.as_deref_mut(), weight, scratch.v4rho2sigma2);
                    add_opt(output.v4rhosigma3.as_deref_mut(), weight, scratch.v4rhosigma3);
                    add_opt(output.v4sigma4.as_deref_mut(), weight, scratch.v4sigma4);
                }
            }
            Family::Mgga => {
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
///   `NEEDS_LAPLACIAN` flag set on aux) + (tau chain if `NEEDS_TAU` flag
///   set on aux). Mixed lapl-tau cross-derivative fields gate on both flags.
///
/// All 70 MGGA caller output fields are zeroed once before the loop.
pub fn evaluate_mixed_mgga(
    functional: &Functional,
    input: &MggaInput,
    order: DerivativeOrder,
    output: &mut MggaOutput,
    workspace: &mut EvaluationWorkspace,
) -> Result<(), LibxcRsError> {
    if workspace.np() != input.np() || workspace.spin() != input.spin() {
        return Err(LibxcRsError::WorkspaceMismatch {
            expected_np: input.np(),
            actual_np: workspace.np(),
            expected_spin: input.spin(),
            actual_spin: workspace.spin(),
        });
    }

    macro_rules! zero_field { ($field:ident) => {
        if let Some(ref mut b) = output.$field { b.fill(0.0); }
    }; }
    zero_field!(zk);
    zero_field!(vrho); zero_field!(vsigma); zero_field!(vlapl); zero_field!(vtau);
    zero_field!(v2rho2); zero_field!(v2rhosigma); zero_field!(v2rholapl); zero_field!(v2rhotau);
    zero_field!(v2sigma2); zero_field!(v2sigmalapl); zero_field!(v2sigmatau);
    zero_field!(v2lapl2); zero_field!(v2lapltau); zero_field!(v2tau2);
    zero_field!(v3rho3); zero_field!(v3rho2sigma); zero_field!(v3rho2lapl); zero_field!(v3rho2tau);
    zero_field!(v3rhosigma2); zero_field!(v3rhosigmalapl); zero_field!(v3rhosigmatau);
    zero_field!(v3rholapl2); zero_field!(v3rholapltau); zero_field!(v3rhotau2);
    zero_field!(v3sigma3); zero_field!(v3sigma2lapl); zero_field!(v3sigma2tau);
    zero_field!(v3sigmalapl2); zero_field!(v3sigmalapltau); zero_field!(v3sigmatau2);
    zero_field!(v3lapl3); zero_field!(v3lapl2tau); zero_field!(v3lapltau2); zero_field!(v3tau3);
    zero_field!(v4rho4); zero_field!(v4rho3sigma); zero_field!(v4rho3lapl); zero_field!(v4rho3tau);
    zero_field!(v4rho2sigma2); zero_field!(v4rho2sigmalapl); zero_field!(v4rho2sigmatau);
    zero_field!(v4rho2lapl2); zero_field!(v4rho2lapltau); zero_field!(v4rho2tau2);
    zero_field!(v4rhosigma3); zero_field!(v4rhosigma2lapl); zero_field!(v4rhosigma2tau);
    zero_field!(v4rhosigmalapl2); zero_field!(v4rhosigmalapltau); zero_field!(v4rhosigmatau2);
    zero_field!(v4rholapl3); zero_field!(v4rholapl2tau); zero_field!(v4rholapltau2); zero_field!(v4rhotau3);
    zero_field!(v4sigma4); zero_field!(v4sigma3lapl); zero_field!(v4sigma3tau);
    zero_field!(v4sigma2lapl2); zero_field!(v4sigma2lapltau); zero_field!(v4sigma2tau2);
    zero_field!(v4sigmalapl3); zero_field!(v4sigmalapl2tau); zero_field!(v4sigmalapltau2); zero_field!(v4sigmatau3);
    zero_field!(v4lapl4); zero_field!(v4lapl3tau); zero_field!(v4lapl2tau2); zero_field!(v4lapltau3); zero_field!(v4tau4);

    for (aux, &weight) in functional
        .auxiliaries
        .iter()
        .zip(functional.mix_coefficients.iter())
    {
        match aux.meta.family {
            Family::Lda => {
                let lda_fn = LdaFunctional::from_id(aux.meta.id)?;
                let lda_input = LdaInput::new(input.rho(), input.np(), input.spin())?;
                workspace.zero_scratch();
                {
                    let scratch = workspace.lda_scratch_mut();
                    let mut aux_output = LdaOutput {
                        zk: Some(scratch.zk),
                        vrho: if order >= DerivativeOrder::Vxc { Some(scratch.vrho) } else { None },
                        v2rho2: if order >= DerivativeOrder::Fxc { Some(scratch.v2rho2) } else { None },
                        v3rho3: if order >= DerivativeOrder::Kxc { Some(scratch.v3rho3) } else { None },
                        v4rho4: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho4) } else { None },
                    };
                    dispatch_lda(lda_fn, &lda_input, order, &mut aux_output, &*aux.params, &aux.thresholds)?;
                }
                let scratch = workspace.lda_scratch_mut();
                add_opt(output.zk.as_deref_mut(), weight, scratch.zk);
                if order >= DerivativeOrder::Vxc {
                    add_opt(output.vrho.as_deref_mut(), weight, scratch.vrho);
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt(output.v2rho2.as_deref_mut(), weight, scratch.v2rho2);
                }
                if order >= DerivativeOrder::Kxc {
                    add_opt(output.v3rho3.as_deref_mut(), weight, scratch.v3rho3);
                }
                if order >= DerivativeOrder::Lxc {
                    add_opt(output.v4rho4.as_deref_mut(), weight, scratch.v4rho4);
                }
            }
            Family::Gga => {
                let gga_fn = GgaFunctional::from_id(aux.meta.id)?;
                let gga_input = GgaInput::new(input.rho(), input.sigma(), input.np(), input.spin())?;
                workspace.zero_scratch();
                {
                    let scratch = workspace.gga_scratch_mut();
                    let mut aux_output = GgaOutput {
                        zk: Some(scratch.zk),
                        vrho: if order >= DerivativeOrder::Vxc { Some(scratch.vrho) } else { None },
                        vsigma: if order >= DerivativeOrder::Vxc { Some(scratch.vsigma) } else { None },
                        v2rho2: if order >= DerivativeOrder::Fxc { Some(scratch.v2rho2) } else { None },
                        v2rhosigma: if order >= DerivativeOrder::Fxc { Some(scratch.v2rhosigma) } else { None },
                        v2sigma2: if order >= DerivativeOrder::Fxc { Some(scratch.v2sigma2) } else { None },
                        v3rho3: if order >= DerivativeOrder::Kxc { Some(scratch.v3rho3) } else { None },
                        v3rho2sigma: if order >= DerivativeOrder::Kxc { Some(scratch.v3rho2sigma) } else { None },
                        v3rhosigma2: if order >= DerivativeOrder::Kxc { Some(scratch.v3rhosigma2) } else { None },
                        v3sigma3: if order >= DerivativeOrder::Kxc { Some(scratch.v3sigma3) } else { None },
                        v4rho4: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho4) } else { None },
                        v4rho3sigma: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho3sigma) } else { None },
                        v4rho2sigma2: if order >= DerivativeOrder::Lxc { Some(scratch.v4rho2sigma2) } else { None },
                        v4rhosigma3: if order >= DerivativeOrder::Lxc { Some(scratch.v4rhosigma3) } else { None },
                        v4sigma4: if order >= DerivativeOrder::Lxc { Some(scratch.v4sigma4) } else { None },
                    };
                    dispatch_gga(gga_fn, &gga_input, order, &mut aux_output, &*aux.params, &aux.thresholds)?;
                }
                let scratch = workspace.gga_scratch_mut();
                add_opt(output.zk.as_deref_mut(), weight, scratch.zk);
                if order >= DerivativeOrder::Vxc {
                    add_opt(output.vrho.as_deref_mut(), weight, scratch.vrho);
                    add_opt(output.vsigma.as_deref_mut(), weight, scratch.vsigma);
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt(output.v2rho2.as_deref_mut(), weight, scratch.v2rho2);
                    add_opt(output.v2rhosigma.as_deref_mut(), weight, scratch.v2rhosigma);
                    add_opt(output.v2sigma2.as_deref_mut(), weight, scratch.v2sigma2);
                }
                if order >= DerivativeOrder::Kxc {
                    add_opt(output.v3rho3.as_deref_mut(), weight, scratch.v3rho3);
                    add_opt(output.v3rho2sigma.as_deref_mut(), weight, scratch.v3rho2sigma);
                    add_opt(output.v3rhosigma2.as_deref_mut(), weight, scratch.v3rhosigma2);
                    add_opt(output.v3sigma3.as_deref_mut(), weight, scratch.v3sigma3);
                }
                if order >= DerivativeOrder::Lxc {
                    add_opt(output.v4rho4.as_deref_mut(), weight, scratch.v4rho4);
                    add_opt(output.v4rho3sigma.as_deref_mut(), weight, scratch.v4rho3sigma);
                    add_opt(output.v4rho2sigma2.as_deref_mut(), weight, scratch.v4rho2sigma2);
                    add_opt(output.v4rhosigma3.as_deref_mut(), weight, scratch.v4rhosigma3);
                    add_opt(output.v4sigma4.as_deref_mut(), weight, scratch.v4sigma4);
                }
            }
            Family::Mgga => {
                let mgga_fn = MggaFunctional::from_id(aux.meta.id)?;
                let needs_lapl = aux.meta.flags.contains(FunctionalFlags::NEEDS_LAPLACIAN);
                let needs_tau = aux.meta.flags.contains(FunctionalFlags::NEEDS_TAU);
                let needs_both = needs_lapl && needs_tau;

                workspace.zero_scratch();
                {
                    let scratch = workspace.mgga_scratch_mut();
                    let mut aux_output = MggaOutput::default();
                    aux_output.zk = Some(scratch.zk);
                    if order >= DerivativeOrder::Vxc {
                        aux_output.vrho = Some(scratch.vrho);
                        aux_output.vsigma = Some(scratch.vsigma);
                        if needs_lapl { aux_output.vlapl = Some(scratch.vlapl); }
                        if needs_tau { aux_output.vtau = Some(scratch.vtau); }
                    }
                    if order >= DerivativeOrder::Fxc {
                        aux_output.v2rho2 = Some(scratch.v2rho2);
                        aux_output.v2rhosigma = Some(scratch.v2rhosigma);
                        aux_output.v2sigma2 = Some(scratch.v2sigma2);
                        if needs_lapl {
                            aux_output.v2rholapl = Some(scratch.v2rholapl);
                            aux_output.v2sigmalapl = Some(scratch.v2sigmalapl);
                            aux_output.v2lapl2 = Some(scratch.v2lapl2);
                        }
                        if needs_tau {
                            aux_output.v2rhotau = Some(scratch.v2rhotau);
                            aux_output.v2sigmatau = Some(scratch.v2sigmatau);
                            aux_output.v2tau2 = Some(scratch.v2tau2);
                        }
                        if needs_both {
                            aux_output.v2lapltau = Some(scratch.v2lapltau);
                        }
                    }
                    // Order >= Kxc / Lxc: dispatch_mgga currently rejects them
                    // upstream, so leave the higher-order aux_output fields as
                    // None. If/when MGGA Fxc+ is wired, expand here.
                    let _ = (needs_lapl, needs_tau, needs_both);
                    dispatch_mgga(mgga_fn, input, order, &mut aux_output, &*aux.params, &aux.thresholds)?;
                }
                let scratch = workspace.mgga_scratch_mut();
                // Always-accumulate (rho-chain, all aux families contribute).
                add_opt(output.zk.as_deref_mut(), weight, scratch.zk);
                if order >= DerivativeOrder::Vxc {
                    add_opt(output.vrho.as_deref_mut(), weight, scratch.vrho);
                    add_opt(output.vsigma.as_deref_mut(), weight, scratch.vsigma);
                    if needs_lapl {
                        add_opt(output.vlapl.as_deref_mut(), weight, scratch.vlapl);
                    }
                    if needs_tau {
                        add_opt(output.vtau.as_deref_mut(), weight, scratch.vtau);
                    }
                }
                if order >= DerivativeOrder::Fxc {
                    add_opt(output.v2rho2.as_deref_mut(), weight, scratch.v2rho2);
                    add_opt(output.v2rhosigma.as_deref_mut(), weight, scratch.v2rhosigma);
                    add_opt(output.v2sigma2.as_deref_mut(), weight, scratch.v2sigma2);
                    if needs_lapl {
                        add_opt(output.v2rholapl.as_deref_mut(), weight, scratch.v2rholapl);
                        add_opt(output.v2sigmalapl.as_deref_mut(), weight, scratch.v2sigmalapl);
                        add_opt(output.v2lapl2.as_deref_mut(), weight, scratch.v2lapl2);
                    }
                    if needs_tau {
                        add_opt(output.v2rhotau.as_deref_mut(), weight, scratch.v2rhotau);
                        add_opt(output.v2sigmatau.as_deref_mut(), weight, scratch.v2sigmatau);
                        add_opt(output.v2tau2.as_deref_mut(), weight, scratch.v2tau2);
                    }
                    if needs_both {
                        add_opt(output.v2lapltau.as_deref_mut(), weight, scratch.v2lapltau);
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
    use crate::eval::dispatch::dispatch_lda;
    use crate::input::LdaInput;
    use crate::model::{DerivativeOrder, Spin, Thresholds};
    use crate::output::LdaOutput;

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
                Some(&mut zk_direct), Some(&mut vrho_direct), None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Vxc, &mut out_direct,
                         &LdaXParams::default(), &default_thresholds()).unwrap();
        }

        // Mixed with single aux, weight=1.0
        let mut zk_mixed = vec![0.0f64; np];
        let mut vrho_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed), Some(&mut vrho_mixed), None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 1.0,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Vxc, &mut out_mixed, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            assert!(
                (zk_mixed[i] - zk_direct[i]).abs() < 1e-15,
                "zk[{i}]: mixed={} vs direct={}", zk_mixed[i], zk_direct[i]
            );
            assert!(
                (vrho_mixed[i] - vrho_direct[i]).abs() < 1e-15,
                "vrho[{i}]: mixed={} vs direct={}", vrho_mixed[i], vrho_direct[i]
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
                Some(&mut zk_direct), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Exc, &mut out_direct,
                         &LdaXParams::default(), &default_thresholds()).unwrap();
        }

        // Mixed with two auxes: 0.7 + 0.3 = 1.0
        let mut zk_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![
                AuxiliaryConfig { alpha: 1.0, weight: 0.7, thresholds: default_thresholds() },
                AuxiliaryConfig { alpha: 1.0, weight: 0.3, thresholds: default_thresholds() },
            ];
            evaluate_mixed_lda(&input, DerivativeOrder::Exc, &mut out_mixed, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            assert!(
                (zk_mixed[i] - zk_direct[i]).abs() < 1e-14,
                "zk[{i}]: mixed={} vs direct={}", zk_mixed[i], zk_direct[i]
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
                Some(&mut zk_direct), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Exc, &mut out_direct,
                         &LdaXParams::default(), &default_thresholds()).unwrap();
        }

        // Mixed with weight=0.5
        let mut zk_mixed = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out_mixed = LdaOutput::new(
                Some(&mut zk_mixed), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            let auxes = vec![AuxiliaryConfig {
                alpha: 1.0,
                weight: 0.5,
                thresholds: default_thresholds(),
            }];
            evaluate_mixed_lda(&input, DerivativeOrder::Exc, &mut out_mixed, &auxes, &mut ws).unwrap();
        }

        for i in 0..np {
            let expected = zk_direct[i] * 0.5;
            assert!(
                (zk_mixed[i] - expected).abs() < 1e-15,
                "zk[{i}]: mixed={} vs expected={}", zk_mixed[i], expected
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
                Some(&mut zk), Some(&mut vrho), None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
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
            let mut out = LdaOutput::new(
                Some(&mut zk), None, None, None, None,
                np, Spin::Unpolarized,
            ).unwrap();
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
                Some(&mut zk_d), Some(&mut vrho_d), Some(&mut v2rho2_d), None, None,
                np, Spin::Unpolarized,
            ).unwrap();
            dispatch_lda(LdaFunctional::LdaX, &input, DerivativeOrder::Fxc, &mut out,
                         &LdaXParams::default(), &default_thresholds()).unwrap();
        }

        // Mixed with weight=1.0
        let mut zk_m = vec![0.0f64; np];
        let mut vrho_m = vec![0.0f64; np];
        let mut v2rho2_m = vec![0.0f64; np];
        let mut ws = EvaluationWorkspace::new(np, Spin::Unpolarized);
        {
            let mut out = LdaOutput::new(
                Some(&mut zk_m), Some(&mut vrho_m), Some(&mut v2rho2_m), None, None,
                np, Spin::Unpolarized,
            ).unwrap();
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
}
