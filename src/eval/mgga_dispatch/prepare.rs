//! Driver-level input regularization for the production MGGA dispatch path.
//!
//! libxc's MGGA "work" driver (`work_mgga.c`) regularizes the kinetic-energy
//! density to its von Weizsäcker lower bound — τ ≥ τ_W = σ/(8ρ) — *before*
//! evaluating the maple2c functional expression. The maple2c expression itself
//! consumes the raw τ it is handed; the clamp lives in the driver, not the
//! kernel.
//!
//! The Rust per-functional dispatch (`dispatch_mgga`) historically passed raw τ
//! straight to the kernel launch, omitting this regularization. For any input
//! with τ < τ_W that produces a SYSTEMIC, percent-level divergence from libxc
//! across every MGGA functional — not an f32/tolerance artifact (gap G-1; see
//! memory `project_translator_missing_workmgga_tau_clamp`, root-caused while
//! debugging the Phase 11.1 G-3 `mgga_c_b94` canary). This module restores the
//! driver-side regularization at the single MGGA chokepoint, so every functional
//! routed through `dispatch_mgga` inherits it.
//!
//! Semantics (mirror libxc):
//! - **Raise-only:** `τ_clamped = τ.max(σ/(8ρ))` — a no-op on points that already
//!   satisfy τ ≥ τ_W; it never lowers a compliant τ.
//! - **ρ → 0 guard:** at `ρ < dens_threshold` the point is below libxc's density
//!   floor (the functional contribution is masked there); raw τ is kept to avoid
//!   a divide-by-zero / NaN from `σ/(8ρ)`.

/// Apply the von Weizsäcker lower bound τ ≥ σ/(8ρ) to the input kinetic-energy
/// density, mirroring libxc's MGGA work-driver regularization.
///
/// Raise-only and guarded at `ρ < dens_threshold` (see module docs). The
/// arithmetic `tau[i].max(sigma[i] / (8.0 * rho[i]))` is byte-identical to the
/// formula proven against the libxc id=397 oracle in
/// `verify-canary/tests/g3_mgga_c_b94_parity.rs` (~5e-13 on 4 sub-vW points).
pub(crate) fn tau_von_weizsacker(
    rho: &[f64],
    sigma: &[f64],
    tau: &[f64],
    dens_threshold: f64,
) -> Vec<f64> {
    (0..tau.len())
        .map(|i| {
            if rho[i] < dens_threshold {
                tau[i]
            } else {
                tau[i].max(sigma[i] / (8.0 * rho[i]))
            }
        })
        .collect()
}
