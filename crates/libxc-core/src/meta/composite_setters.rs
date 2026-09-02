//! libxc `set_ext_params` functions that do more than copy.
//!
//! # Why this table exists, and why it is hand-written
//!
//! [`PropagationRule`](super::PropagationRule) covers the common shape: a
//! hybrid parent's named ext_param is *copied* to a named ext_param on one of
//! its auxiliaries. `cargo xtask generate-metadata` emits those, and
//! deliberately **rejects** any setter that transforms values instead of
//! copying them, so `generated_propagation.rs` is guaranteed to be pure copies.
//!
//! That rejection left a hole. Some libxc setters assign a *constant*, a
//! *negated* parent value, or write into `mix_coef` rather than into an
//! auxiliary's ext_params — none of which a copy rule can say. The HSE family
//! is the case that matters most, and the hole made it numerically wrong:
//!
//! ```c
//! /* libxc-master/src/hyb_gga_xc_hse.c: hse03_set_ext_params */
//! beta      = get_ext_param(p, ext_params, 0);
//! omega_HF  = get_ext_param(p, ext_params, 1);
//! omega_PBE = get_ext_param(p, ext_params, 2);
//!
//! p->mix_coef[1] = -beta;
//!
//! p->hyb_coeff[0] = beta;
//! p->hyb_omega[0] = omega_HF;
//!
//! xc_func_set_ext_params_name(p->func_aux[0], "_omega", 0.0);
//! xc_func_set_ext_params_name(p->func_aux[1], "_omega", omega_PBE);
//! ```
//!
//! HSE06 is `1.0*wpbeh(w=0) - beta*wpbeh(w=omega_PBE) + 1.0*PBEc`. Without the
//! last line, `func_aux[1]` keeps `gga_x_wpbeh`'s own default `_omega = 0.0`,
//! both legs evaluate the *same* unscreened function, and the whole thing
//! collapses to `(1 - beta)*wpbeh(0) + PBEc` — a PBE0-shaped semilocal part
//! with no screening anywhere in it. That is what this tree computed for
//! HSE03/06/12/12s/sol before this table existed.
//!
//! It is hand-written rather than generated because scraping arbitrary C
//! assignment statements is exactly the kind of guessing the rest of the
//! pipeline refuses to do. Each entry below is a transcription of one C line,
//! with that line quoted next to it, so it can be checked by reading rather
//! than trusted.
//!
//! # Relationship to the auxiliary weights already in the metadata
//!
//! `FunctionalMeta::auxiliaries` already carries `mix_coef` evaluated at the
//! *default* ext_params (HSE06's second entry is `-0.25`, which is `-beta` for
//! the default `beta = 0.25`). So for a functional left at its defaults the
//! `MixCoefficient` rules below reproduce what is already there, and only bite
//! when a caller changes `_beta`. The `AuxExtParam` rules are the ones that
//! change today's numbers.

use crate::model::FunctionalId;

/// Where a composite setter's assignment comes from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetterSource {
    /// A literal written into the C setter (`..., "_omega", 0.0`).
    Constant(f64),
    /// `scale * parent_ext_params[index]`.
    ///
    /// `scale` is `-1.0` for libxc's `p->mix_coef[1] = -beta` and `1.0` for a
    /// plain forward of a parent value.
    ParentParam { index: u16, scale: f64 },
}

/// What a composite setter's assignment writes to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SetterTarget {
    /// The named ext_param of `func_aux[slot]`.
    AuxExtParam { slot: u8, name: &'static str },
    /// `mix_coef[slot]` — the weight this auxiliary enters the sum with.
    MixCoefficient { slot: u8 },
}

/// One assignment performed by a libxc `set_ext_params` function.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CompositeSetterRule {
    pub parent_id: FunctionalId,
    pub target: SetterTarget,
    pub source: SetterSource,
}

/// The HSE family shares one setter (`hse03_set_ext_params`) and one init
/// (`hyb_gga_xc_hse_init`), so all five ids get the same three assignments.
///
/// `hyb_gga_xc_hse_init` builds the mix as
/// `{XC_GGA_X_WPBEH, XC_GGA_X_WPBEH, XC_GGA_C_PBE}` with coefficients
/// `{1.0, 0.0, 1.0}`, and the comment there says outright that
/// `funcs_coef[1] will be set by set_ext_params`.
const fn hse(parent: u16) -> [CompositeSetterRule; 3] {
    [
        // p->mix_coef[1] = -beta;
        CompositeSetterRule {
            parent_id: FunctionalId(parent),
            target: SetterTarget::MixCoefficient { slot: 1 },
            source: SetterSource::ParentParam { index: 0, scale: -1.0 },
        },
        // xc_func_set_ext_params_name(p->func_aux[0], "_omega", 0.0);
        CompositeSetterRule {
            parent_id: FunctionalId(parent),
            target: SetterTarget::AuxExtParam { slot: 0, name: "_omega" },
            source: SetterSource::Constant(0.0),
        },
        // xc_func_set_ext_params_name(p->func_aux[1], "_omega", omega_PBE);
        CompositeSetterRule {
            parent_id: FunctionalId(parent),
            target: SetterTarget::AuxExtParam { slot: 1, name: "_omega" },
            source: SetterSource::ParentParam { index: 2, scale: 1.0 },
        },
    ]
}

/// XC_HYB_GGA_XC_HSE03
const HSE03: [CompositeSetterRule; 3] = hse(427);
/// XC_HYB_GGA_XC_HSE06
const HSE06: [CompositeSetterRule; 3] = hse(428);
/// XC_HYB_GGA_XC_HSE12
const HSE12: [CompositeSetterRule; 3] = hse(479);
/// XC_HYB_GGA_XC_HSE12S
const HSE12S: [CompositeSetterRule; 3] = hse(480);
/// XC_HYB_GGA_XC_HSE_SOL is **not** built by `hse()`.
///
/// It declares zero ext_params (`{0, NULL, NULL, NULL, NULL}`), mixes
/// `gga_x_hjs_pbe_sol` rather than `gga_x_wpbeh`, and does its assignments in
/// its own init with the screening baked in:
///
/// ```c
/// /* hyb_gga_xc_hse_sol_init */
/// int funcs_id[3] = {XC_GGA_X_HJS_PBE_SOL, XC_GGA_X_HJS_PBE_SOL, XC_GGA_C_PBE};
/// double funcs_coef[3] = {1.0, -0.25, 1.0};
/// xc_mix_init(p, 3, funcs_id, funcs_coef);
/// xc_hyb_init_sr(p, 0.25, 0.11);
/// xc_func_set_ext_params_name(p->func_aux[0], "_omega", 0.0);
/// xc_func_set_ext_params_name(p->func_aux[1], "_omega", p->hyb_omega[0]);
/// ```
///
/// `hyb_omega[0]` is the 0.11 from `xc_hyb_init_sr`, so both assignments are
/// constants and there is no `mix_coef` rule -- the -0.25 is fixed in the init
/// and already sits in `FunctionalMeta::auxiliaries`.
const HSE_SOL: [CompositeSetterRule; 2] = [
    CompositeSetterRule {
        parent_id: FunctionalId(481),
        target: SetterTarget::AuxExtParam { slot: 0, name: "_omega" },
        source: SetterSource::Constant(0.0),
    },
    CompositeSetterRule {
        parent_id: FunctionalId(481),
        target: SetterTarget::AuxExtParam { slot: 1, name: "_omega" },
        source: SetterSource::Constant(0.11),
    },
];

/// Every composite-setter assignment this crate knows how to reproduce.
///
/// A functional absent from this table and from `PROPAGATION_RULES` simply has
/// no parent-to-aux flow, which is the common case.
pub const COMPOSITE_SETTER_RULES: &[CompositeSetterRule] = &{
    let mut out = [HSE03[0]; 14];
    let mut i = 0;
    while i < 3 {
        out[i] = HSE03[i];
        out[3 + i] = HSE06[i];
        out[6 + i] = HSE12[i];
        out[9 + i] = HSE12S[i];
        i += 1;
    }
    out[12] = HSE_SOL[0];
    out[13] = HSE_SOL[1];
    out
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hse06_has_the_three_assignments_from_libxc() {
        let rules: Vec<_> = COMPOSITE_SETTER_RULES
            .iter()
            .filter(|r| r.parent_id == FunctionalId(428))
            .collect();
        assert_eq!(rules.len(), 3, "hse03_set_ext_params makes three assignments");

        // p->mix_coef[1] = -beta, beta being ext_params[0].
        assert!(rules.iter().any(|r| r.target
            == SetterTarget::MixCoefficient { slot: 1 }
            && r.source == SetterSource::ParentParam { index: 0, scale: -1.0 }));

        // func_aux[0] is the unscreened leg: omega pinned to zero.
        assert!(rules.iter().any(|r| r.target
            == SetterTarget::AuxExtParam { slot: 0, name: "_omega" }
            && r.source == SetterSource::Constant(0.0)));

        // func_aux[1] is the screened leg: omega comes from _omega_PBE,
        // which is ext_params index 2, NOT index 1 (_omega_HF, which is the
        // exact-exchange screening the caller applies outside the functional).
        assert!(rules.iter().any(|r| r.target
            == SetterTarget::AuxExtParam { slot: 1, name: "_omega" }
            && r.source == SetterSource::ParentParam { index: 2, scale: 1.0 }));
    }

    #[test]
    fn every_hse_variant_is_covered() {
        // 427/428/479/480 share `hse03_set_ext_params`: three assignments each.
        for id in [427u16, 428, 479, 480] {
            let n = COMPOSITE_SETTER_RULES
                .iter()
                .filter(|r| r.parent_id == FunctionalId(id))
                .count();
            assert_eq!(n, 3, "id {id} should carry the shared hse setter");
        }
        // 481 (HSEsol) has no ext_params and does its two assignments in its
        // own init, so it carries two constant rules and no mix_coef rule.
        let sol: Vec<_> = COMPOSITE_SETTER_RULES
            .iter()
            .filter(|r| r.parent_id == FunctionalId(481))
            .collect();
        assert_eq!(sol.len(), 2);
        assert!(sol.iter().all(|r| matches!(r.source, SetterSource::Constant(_))));
        assert_eq!(COMPOSITE_SETTER_RULES.len(), 14);
    }

    /// A rule reading a parent ext_param on a functional that has none would
    /// fail at `Functional::new` time, which is how the first draft of this
    /// table (which wrongly gave HSEsol the shared setter) was caught.
    #[test]
    fn parent_param_rules_only_target_functionals_with_ext_params() {
        use crate::registry::lookup_by_id;
        for r in COMPOSITE_SETTER_RULES {
            if let SetterSource::ParentParam { index, .. } = r.source {
                let meta = lookup_by_id(r.parent_id.0).expect("known id");
                assert!(
                    (index as usize) < meta.ext_params.len(),
                    "{} rule reads ext_param {index} but the functional declares {}",
                    meta.name,
                    meta.ext_params.len()
                );
            }
        }
    }
}
