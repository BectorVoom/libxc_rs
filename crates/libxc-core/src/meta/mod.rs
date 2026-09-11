pub(crate) mod generated;
pub(crate) mod generated_hybrid;
pub mod composite_setters;
pub mod generated_aux_overrides;
pub mod generated_deorbitalized;
pub mod generated_propagation;
pub use generated_aux_overrides::{AUX_EXT_PARAM_OVERRIDES, AuxExtParamOverride};
pub use composite_setters::{
    COMPOSITE_SETTER_RULES, CompositeSetterRule, SetterSource, SetterTarget,
};
pub use generated_propagation::PROPAGATION_RULES;

use crate::model::{
    DerivativeOrder, Family, FunctionalFlags, FunctionalId, HybridTermKind, HybridType, Kind,
};

/// Literature reference
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reference {
    pub citation: &'static str,
    pub doi: &'static str,
    pub bibtex: &'static str,
    pub key: &'static str,
}

/// External parameter specification
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtParamSpec {
    pub name: &'static str,
    pub description: &'static str,
    pub default_value: f64,
    /// If true, this is an internal parameter (name starts with '_')
    pub is_internal: bool,
}

/// A single hybrid exchange term
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HybridTerm {
    pub kind: HybridTermKind,
    pub coefficient: f64,
    pub omega: f64,
}

/// Copy-style ext_param flow from a hybrid parent's named ext_param to a
/// named ext_param on one of its auxiliary functionals.
/// Emitted by `cargo xtask generate-metadata` (D-16). Non-Copy transforms
/// are rejected at xtask time per D-16 / Plan 05-01 Task 2 acceptance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PropagationRule {
    pub parent_id: FunctionalId,
    pub parent_param_name: &'static str,
    pub parent_param_index: u16,
    pub aux_slot: u8,
    pub aux_param_name: &'static str,
}

/// Static metadata for a functional. Lives in .rodata.
#[derive(Debug, PartialEq)]
pub struct FunctionalMeta {
    pub id: FunctionalId,
    pub name: &'static str,
    pub kind: Kind,
    pub family: Family,
    pub flags: FunctionalFlags,
    pub references: &'static [Reference],
    pub ext_params: &'static [ExtParamSpec],
    pub default_density_threshold: f64,
    /// Auxiliary functional IDs and weights for mixed/hybrid functionals
    pub auxiliaries: &'static [(FunctionalId, f64)],
    /// Hybrid term definitions
    pub hybrid_terms: &'static [HybridTerm],
    /// Non-local correlation parameters (b, C) if applicable
    pub nlc_params: Option<(f64, f64)>,
    /// Maximum supported derivative order
    pub max_order: DerivativeOrder,
    /// Hybrid type classification
    pub hybrid_type: HybridType,
}

pub use generated_deorbitalized::DEORBITALIZED;

/// A deorbitalized meta-GGA: libxc's `xc_deorbitalize_init(p, base, ked)`.
///
/// libxc evaluates one (`xc_deorbitalize_func`, `deorbitalize_func.c`) by
/// computing a kinetic energy density from `ked`, `tau = rho * e_ked(rho,
/// sigma, lapl)`, feeding that `tau` to `base`, and chain-ruling every
/// derivative through it. That is an init call rather than an `xc_mix_init`,
/// so [`FunctionalMeta::auxiliaries`] does not record it; [`DEORBITALIZED`]
/// does, read out of libxc's own `xc_func_type` by
/// `verify/tests/gen_deorbitalized.rs`.
#[derive(Debug, Clone, Copy)]
pub struct Deorbitalized {
    pub id: FunctionalId,
    /// The meta-GGA evaluated at the deorbitalized `tau` (libxc's `func_aux[0]`).
    pub base: FunctionalId,
    /// The kinetic-energy functional that stands in for `tau` (`func_aux[1]`).
    pub ked: FunctionalId,
    /// For each of the parent's ext_params, in order, the auxiliary it is
    /// copied to and its index there: slot 0 is `base`, slot 1 is `ked`.
    pub ext_to_aux: &'static [(u8, u8)],
}

/// The deorbitalization record for `id`, if it is one.
pub fn deorbitalized(id: FunctionalId) -> Option<&'static Deorbitalized> {
    DEORBITALIZED.iter().find(|d| d.id == id)
}

/// `lda_k_gds08_worker`'s id as the composites that mix it record it.
///
/// libxc numbers the worker 100001 and declares it in `xc_funcs_worker.h`,
/// outside its public header. The four public composites that mix it
/// (`gga_k_gds08`, `gga_k_ghds10`, `gga_k_ghds10r`, `gga_k_tkvln`) carry it
/// in their generated auxiliary lists as 100001 truncated to the `u16`
/// [`FunctionalId`], which is 34465, so this is that value. No public libxc
/// id is anywhere near it.
pub const LDA_K_GDS08_WORKER: FunctionalId = FunctionalId((100_001u32 & 0xffff) as u16);

/// Named and laid out like the generated `*_EXT_PARAMS` tables on purpose:
/// `tools/translate_rayon/extract_params.py` reads it the same way, and
/// refuses runtime ext_params for the worker unless every default here lands
/// bit for bit on the kernel default it feeds.
#[rustfmt::skip]
static XC_LDA_K_GDS08_WORKER_EXT_PARAMS: &[ExtParamSpec] = &[
    ExtParamSpec { name: "_A", description: "linear term", default_value: 0.860, is_internal: true },
    ExtParamSpec { name: "_B", description: "term proportional to the logarithm of the density", default_value: 0.224, is_internal: true },
    ExtParamSpec { name: "_C", description: "term proportional to the square of the logarithm", default_value: 0.0, is_internal: true },
];

/// `xc_func_info_lda_k_gds08_worker` (`lda_k_gds08_worker.c`), field for
/// field. `verify/tests/composite_oracle.rs` checks it against libxc's own
/// info block.
static GDS08_WORKER: FunctionalMeta = FunctionalMeta {
    id: LDA_K_GDS08_WORKER,
    name: "XC_LDA_K_GDS08_WORKER",
    kind: Kind::Kinetic,
    family: Family::Lda,
    flags: FunctionalFlags::DIM_3D.union(FunctionalFlags::HAVE_ALL),
    references: &[],
    ext_params: XC_LDA_K_GDS08_WORKER_EXT_PARAMS,
    default_density_threshold: 1e-15,
    auxiliaries: &[],
    hybrid_terms: &[],
    nlc_params: None,
    max_order: DerivativeOrder::Lxc,
    hybrid_type: HybridType::Semilocal,
};

/// The number libxc itself uses for `id`: the id unchanged, except for an
/// internal worker whose libxc number does not fit the `u16` id
/// (`lda_k_gds08_worker` is 100001, held here as [`LDA_K_GDS08_WORKER`]).
/// What the C ABI must report, e.g. from `xc_aux_func_ids`.
pub fn libxc_number(id: FunctionalId) -> i32 {
    if id == LDA_K_GDS08_WORKER { 100_001 } else { i32::from(id.raw()) }
}

/// Metadata for one of libxc's internal worker functionals, which only a
/// composite's auxiliary list reaches.
///
/// **Not a registry lookup, and deliberately so.** A functional without a
/// public libxc id is not part of the API: [`crate::registry`] does not know
/// these, so they cannot be constructed or evaluated on their own. They exist
/// so the public composites that mix them can be.
pub fn internal_auxiliary(id: FunctionalId) -> Option<&'static FunctionalMeta> {
    (id == LDA_K_GDS08_WORKER).then_some(&GDS08_WORKER)
}
