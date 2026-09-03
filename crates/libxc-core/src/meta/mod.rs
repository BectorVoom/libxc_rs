pub(crate) mod generated;
pub(crate) mod generated_hybrid;
pub mod composite_setters;
pub mod generated_aux_overrides;
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
