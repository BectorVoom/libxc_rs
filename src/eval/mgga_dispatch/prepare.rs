//! Driver-level input regularization for the production MGGA dispatch path.
//!
//! libxc's MGGA "work" driver (`work_mgga_inc.c:54-68`) regularizes the inputs
//! *before* evaluating the maple2c functional expression. The maple2c expression
//! itself consumes whatever it is handed; the regularization lives in the driver,
//! not the kernel.
//!
//! The exact libxc sequence (unpolarized branch, with
//! `XC_ENFORCE_FERMI_HOLE_CURVATURE` compiled into the vendored oracle) is:
//!
//! ```text
//!   if (rho < dens_threshold) continue;                       // low-density screen
//!   my_rho   = max(dens_threshold, rho);                      // 1. rho floor
//!   my_sigma = max(sigma_threshold^2, sigma);                 // 2. sigma FLOOR
//!   my_tau   = max(tau_threshold, tau);                       // 3. tau FLOOR
//!   my_sigma = min(my_sigma, 8.0 * my_rho * my_tau);          // 4. sigma-DOWN clamp
//! ```
//!
//! Step 4 is the Fermi-hole curvature constraint: the curvature
//! `1 - sigma/(8*rho*tau)` must stay non-negative, so sigma is clamped DOWN to
//! `8*rho*tau`. It consumes the ALREADY-FLOORED `my_rho` and `my_tau`.
//!
//! ## Why this supersedes the Phase-11 G-1 tau-up clamp (D-01/D-02)
//!
//! The earlier von-Weizsäcker clamp raised τ UP (`τ ← max(τ, σ/(8ρ))`). That
//! enforces the SAME boundary `σ ≤ 8ρτ`, but it feeds a DIFFERENT `(ρ, σ, τ)`
//! triple to any functional that reads σ and τ independently — the prime suspect
//! for the 5-functional small-error MGGA cluster. Mirroring libxc's σ-DOWN clamp
//! exactly (this module) restores input-level byte parity with the C oracle.
//!
//! The σ-down clamp is also strictly SAFER than the old τ-up clamp: it is a
//! multiply-and-`min`, with no `σ/(8ρ)` division and therefore no divide-by-zero
//! risk as `ρ → 0`.

/// Mirror libxc's MGGA work-driver input regularization (`work_mgga_inc.c:54-68`,
/// unpolarized branch, `XC_ENFORCE_FERMI_HOLE_CURVATURE` active in the vendored
/// oracle).
///
/// Ordered sequence (FP-order locked per CLAUDE.md — DO NOT reorder):
///   1. rho floor:   `my_rho   = max(dens_threshold, rho)`
///   2. sigma floor: `my_sigma = max(sigma_threshold^2, sigma)` where
///      `sigma_threshold = dens_threshold^(4/3)`
///   3. tau floor:   `my_tau   = max(tau_threshold, tau)` (`tau_threshold = 1e-20`)
///   4. sigma-DOWN Fermi-hole clamp: `my_sigma = min(my_sigma, 8.0 * my_rho * my_tau)`
///
/// **`needs_tau` gate (libxc `work_mgga_inc.c:62`):** steps 3 and 4 — and the rho floor
/// they consume — apply ONLY when the functional declares `XC_FLAGS_NEEDS_TAU`. The sigma
/// floor (step 2) is unconditional. A non-NEEDS_TAU functional (kinetic `mgga_k_*`, or a
/// laplacian-only xc functional) keeps its RAW tau and an UN-clamped sigma: it does not
/// read tau, but it DOES read sigma, so clamping sigma down would feed it a value libxc
/// never produces (the regression the `mgga_oracle` gate caught for mgga_k_gea2/gea4 and
/// mgga_xc_zlp/lp90).
///
/// The clamp at step 4 consumes the ALREADY-FLOORED `my_rho` and `my_tau` (not raw
/// values). Returns the regularized `(sigma, tau)` — BOTH must flow into the kernel
/// launch. A point with `rho < dens_threshold` is left at its raw `(sigma, tau)` —
/// libxc `continue`s it (`work_mgga_inc.c:54`); the functional contribution is
/// masked there.
///
/// libxc's finiteness re-evaluation fallback (`work_mgga_inc.c:96+`) is INSIDE
/// `#ifdef XC_DEBUG` — it is NOT compiled into the production oracle (D-12), so no
/// such fallback is added here; it is unnecessary for 1e-12 exc parity.
pub(crate) fn regularize_inputs(
    rho: &[f64],
    sigma: &[f64],
    tau: &[f64],
    dens_threshold: f64,
    tau_threshold: f64,
    needs_tau: bool,
) -> (Vec<f64>, Vec<f64>) {
    let sigma_threshold = dens_threshold.powf(4.0 / 3.0);
    let sigma_floor = sigma_threshold * sigma_threshold;
    let mut sigma_out = Vec::with_capacity(sigma.len());
    let mut tau_out = Vec::with_capacity(tau.len());
    for i in 0..rho.len() {
        if rho[i] < dens_threshold {
            // Below the density floor — libxc continues; keep raw inputs.
            sigma_out.push(sigma[i]);
            tau_out.push(tau[i]);
            continue;
        }
        let my_sigma = sigma_floor.max(sigma[i]); // step 2 — sigma floor (always)
        // Steps 1 (rho floor), 3 (tau floor) and 4 (sigma-DOWN clamp) apply ONLY when the
        // functional reads tau — libxc gates them on XC_FLAGS_NEEDS_TAU (work_mgga_inc.c:62).
        // A non-NEEDS_TAU functional (kinetic mgga_k_*, laplacian-only xc) must keep its raw
        // tau and an UN-clamped sigma: clamping sigma down would corrupt a value it reads.
        let (my_sigma, my_tau) = if needs_tau {
            let my_rho = dens_threshold.max(rho[i]); // step 1 — rho floor (consumed by the clamp)
            let my_tau = tau_threshold.max(tau[i]); // step 3 — tau floor
            (my_sigma.min(8.0 * my_rho * my_tau), my_tau) // step 4 — sigma-DOWN
        } else {
            (my_sigma, tau[i])
        };
        sigma_out.push(my_sigma);
        tau_out.push(my_tau);
    }
    (sigma_out, tau_out)
}
