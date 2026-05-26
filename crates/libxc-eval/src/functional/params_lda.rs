//! Per-functional `FunctionalParams` impls for LDA functionals.
//!
//! Every LDA functional enum variant (`LdaFunctional`) has a dedicated
//! zero- or one-ext-param `*Params` struct that implements
//! [`FunctionalParams`]. Six exchange variants (`LdaXParams`,
//! `LdaX2dParams`, `LdaXRelParams`, `LdaXErfParams`, `LdaXSlocParams`,
//! `LdaXYukawaParams`) carry a single `alpha` scalar (Slater exchange
//! scaling, default 1.0) so dispatch arms can downcast and recover the
//! per-functional scaling without plumbing a global knob.
//!
//! The remaining 32 variants carry no runtime-configurable ext_params:
//! their dispatch arms consume hardcoded libxc 7.0.0 defaults (matching
//! the C oracle). Each of those still gets a distinct marker struct +
//! [`FunctionalParams`] impl so `Functional::new` can hand the dispatch
//! layer a typed `&dyn FunctionalParams`.
//!
//! See [`crate::functional::params::FunctionalParams`] for the trait
//! shape and [`crate::functional::lifecycle::construct_params`] for the
//! dispatch table mapping `FunctionalId → Box<dyn FunctionalParams>`.

use std::any::Any;

use libxc_core::error::LibxcRsError;
use crate::functional::params::FunctionalParams;
use libxc_core::model::FunctionalId;

// ============================================================================
// Exchange functionals (6 concrete structs carrying `alpha`)
// ============================================================================

/// Slater exchange scaling for `lda_x` (XC_LDA_X = id 1). Defaults to 1.0.
///
/// This is the authoritative `LdaXParams` impl used by both direct dispatch
/// (via downcast in `dispatch_lda`) and by the mixed-LDA accumulator in
/// `src/eval/mix.rs`.
#[derive(Debug, Clone)]
pub struct LdaXParams {
    raw: Box<[f64]>,
    pub(crate) alpha: f64,
}

impl LdaXParams {
    /// libxc 7.0.0 default for `lda_x`: alpha = 1.0 (Slater).
    pub fn from_defaults() -> Self {
        let raw = Box::<[f64]>::from([1.0]);
        Self { alpha: raw[0], raw }
    }

    /// Construct with an explicit alpha value (used by hybrid mixing).
    pub fn new(alpha: f64) -> Self {
        let raw = Box::<[f64]>::from([alpha]);
        Self { alpha, raw }
    }
}

impl Default for LdaXParams {
    fn default() -> Self {
        Self::from_defaults()
    }
}

impl FunctionalParams for LdaXParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        &self.raw
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(1),
                expected: 1,
                actual: vals.len(),
            });
        }
        self.raw = Box::<[f64]>::from(vals);
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Ext_params carrier for `lda_x_2d` (id 19). Carries Slater exchange
/// scaling `alpha` (libxc default 1.0).
#[derive(Debug, Clone)]
pub struct LdaX2dParams {
    raw: Box<[f64]>,
    #[allow(dead_code)]
    pub(crate) alpha: f64,
}

impl LdaX2dParams {
    pub fn from_defaults() -> Self {
        let raw = Box::<[f64]>::from([1.0]);
        Self { alpha: raw[0], raw }
    }
}

impl Default for LdaX2dParams {
    fn default() -> Self {
        Self::from_defaults()
    }
}

impl FunctionalParams for LdaX2dParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        &self.raw
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(19),
                expected: 1,
                actual: vals.len(),
            });
        }
        self.raw = Box::<[f64]>::from(vals);
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Ext_params carrier for `lda_x_rel` (id 532). Carries Slater exchange
/// scaling `alpha` (libxc default 1.0).
#[derive(Debug, Clone)]
pub struct LdaXRelParams {
    raw: Box<[f64]>,
    #[allow(dead_code)]
    pub(crate) alpha: f64,
}

impl LdaXRelParams {
    pub fn from_defaults() -> Self {
        let raw = Box::<[f64]>::from([1.0]);
        Self { alpha: raw[0], raw }
    }
}

impl Default for LdaXRelParams {
    fn default() -> Self {
        Self::from_defaults()
    }
}

impl FunctionalParams for LdaXRelParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        &self.raw
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(532),
                expected: 1,
                actual: vals.len(),
            });
        }
        self.raw = Box::<[f64]>::from(vals);
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Ext_params carrier for `lda_x_erf` (id 546). Carries Slater exchange
/// scaling `alpha` (libxc default 1.0).
#[derive(Debug, Clone)]
pub struct LdaXErfParams {
    raw: Box<[f64]>,
    #[allow(dead_code)]
    pub(crate) alpha: f64,
}

impl LdaXErfParams {
    pub fn from_defaults() -> Self {
        let raw = Box::<[f64]>::from([1.0]);
        Self { alpha: raw[0], raw }
    }
}

impl Default for LdaXErfParams {
    fn default() -> Self {
        Self::from_defaults()
    }
}

impl FunctionalParams for LdaXErfParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        &self.raw
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(546),
                expected: 1,
                actual: vals.len(),
            });
        }
        self.raw = Box::<[f64]>::from(vals);
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Ext_params carrier for `lda_x_sloc` (id 692). Carries Slater exchange
/// scaling `alpha` (libxc default 1.0).
#[derive(Debug, Clone)]
pub struct LdaXSlocParams {
    raw: Box<[f64]>,
    #[allow(dead_code)]
    pub(crate) alpha: f64,
}

impl LdaXSlocParams {
    pub fn from_defaults() -> Self {
        let raw = Box::<[f64]>::from([1.0]);
        Self { alpha: raw[0], raw }
    }
}

impl Default for LdaXSlocParams {
    fn default() -> Self {
        Self::from_defaults()
    }
}

impl FunctionalParams for LdaXSlocParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        &self.raw
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(692),
                expected: 1,
                actual: vals.len(),
            });
        }
        self.raw = Box::<[f64]>::from(vals);
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Ext_params carrier for `lda_x_yukawa` (id 641). Carries Slater exchange
/// scaling `alpha` (libxc default 1.0).
#[derive(Debug, Clone)]
pub struct LdaXYukawaParams {
    raw: Box<[f64]>,
    #[allow(dead_code)]
    pub(crate) alpha: f64,
}

impl LdaXYukawaParams {
    pub fn from_defaults() -> Self {
        let raw = Box::<[f64]>::from([1.0]);
        Self { alpha: raw[0], raw }
    }
}

impl Default for LdaXYukawaParams {
    fn default() -> Self {
        Self::from_defaults()
    }
}

impl FunctionalParams for LdaXYukawaParams {
    fn ext_param_count(&self) -> usize {
        1
    }

    fn raw_ext_params(&self) -> &[f64] {
        &self.raw
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if vals.len() != 1 {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(641),
                expected: 1,
                actual: vals.len(),
            });
        }
        self.raw = Box::<[f64]>::from(vals);
        self.alpha = vals[0];
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

// ============================================================================
// Zero-ext_params LDA variants (32 marker structs)
// ============================================================================

/// Zero-ext_params marker for `lda_c_rpa` (id 3). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCRpaParams;

impl FunctionalParams for LdaCRpaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(3),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_hl` (id 4). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCHlParams;

impl FunctionalParams for LdaCHlParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(4),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_vwn` (id 7). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCVwnParams;

impl FunctionalParams for LdaCVwnParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(7),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_vwn_rpa` (id 8). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCVwnRpaParams;

impl FunctionalParams for LdaCVwnRpaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(8),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_vwn_1` (id 28). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCVwn1Params;

impl FunctionalParams for LdaCVwn1Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(28),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_vwn_2` (id 29). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCVwn2Params;

impl FunctionalParams for LdaCVwn2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(29),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_vwn_3` (id 30). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCVwn3Params;

impl FunctionalParams for LdaCVwn3Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(30),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_vwn_4` (id 31). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCVwn4Params;

impl FunctionalParams for LdaCVwn4Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(31),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_pz` (id 9). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCPzParams;

impl FunctionalParams for LdaCPzParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(9),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_pw` (id 12). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCPwParams;

impl FunctionalParams for LdaCPwParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(12),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_wigner` (id 2). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCWignerParams;

impl FunctionalParams for LdaCWignerParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(2),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_rc04` (id 27). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCRc04Params;

impl FunctionalParams for LdaCRc04Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(27),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_2d_amgb` (id 15). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaC2dAmgbParams;

impl FunctionalParams for LdaC2dAmgbParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(15),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_2d_prm` (id 16). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaC2dPrmParams;

impl FunctionalParams for LdaC2dPrmParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(16),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_1d_csc` (id 18). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaC1dCscParams;

impl FunctionalParams for LdaC1dCscParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(18),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_1d_loos` (id 26). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaC1dLoosParams;

impl FunctionalParams for LdaC1dLoosParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(26),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_gk72` (id 578). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCGk72Params;

impl FunctionalParams for LdaCGk72Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(578),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_gombas` (id 24). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCGombasParams;

impl FunctionalParams for LdaCGombasParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(24),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_lp96` (id 289). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCLp96Params;

impl FunctionalParams for LdaCLp96Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(289),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_ml1` (id 22). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCMl1Params;

impl FunctionalParams for LdaCMl1Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(22),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_w20` (id 317). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCW20Params;

impl FunctionalParams for LdaCW20Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(317),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_chachiyo` (id 287). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCChachiyoParams;

impl FunctionalParams for LdaCChachiyoParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(287),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_c_chachiyo_mod` (id 307). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaCChachiyoModParams;

impl FunctionalParams for LdaCChachiyoModParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(307),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_k_tf` (id 50). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaKTfParams;

impl FunctionalParams for LdaKTfParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(50),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_k_zlp` (id 550). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaKZlpParams;

impl FunctionalParams for LdaKZlpParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(550),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_xc_teter93` (id 20). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaXcTeter93Params;

impl FunctionalParams for LdaXcTeter93Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(20),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_xc_zlp` (id 43). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaXcZlpParams;

impl FunctionalParams for LdaXcZlpParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(43),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_xc_tih` (id 599). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaXcTihParams;

impl FunctionalParams for LdaXcTihParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(599),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_xc_1d_ehwlrg_1` (id 536). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaXc1dEhwlrg1Params;

impl FunctionalParams for LdaXc1dEhwlrg1Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(536),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_xc_1d_ehwlrg_2` (id 537). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaXc1dEhwlrg2Params;

impl FunctionalParams for LdaXc1dEhwlrg2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(537),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `lda_xc_1d_ehwlrg_3` (id 538). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct LdaXc1dEhwlrg3Params;

impl FunctionalParams for LdaXc1dEhwlrg3Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(538),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Zero-ext_params marker for `hyb_lda_xc_bn05` (id 588). Dispatch uses hardcoded libxc defaults.
#[derive(Debug, Default, Clone, Copy)]
pub struct HybLdaXcBn05Params;

impl FunctionalParams for HybLdaXcBn05Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(588),
                expected: 0,
                actual: vals.len(),
            });
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functional::params::NoParams;

    #[test]
    fn lda_x_params_default_alpha_one() {
        let p = LdaXParams::from_defaults();
        assert_eq!(p.alpha, 1.0);
        assert_eq!(p.ext_param_count(), 1);
        assert_eq!(p.raw_ext_params(), &[1.0]);
    }

    #[test]
    fn lda_x_params_new_stores_alpha() {
        let p = LdaXParams::new(0.7);
        assert_eq!(p.alpha, 0.7);
        assert_eq!(p.raw_ext_params(), &[0.7]);
    }

    #[test]
    fn lda_x_params_set_alpha_writes_through() {
        let mut p = LdaXParams::from_defaults();
        p.set_ext_params(&[0.7]).unwrap();
        assert_eq!(p.alpha, 0.7);
        assert_eq!(p.raw_ext_params(), &[0.7]);
    }

    #[test]
    fn lda_x_params_set_wrong_length_errors() {
        let mut p = LdaXParams::from_defaults();
        let result = p.set_ext_params(&[1.0, 2.0]);
        match result {
            Err(LibxcRsError::ExtParamCountMismatch { expected, actual, .. }) => {
                assert_eq!(expected, 1);
                assert_eq!(actual, 2);
            }
            other => panic!("expected ExtParamCountMismatch, got {other:?}"),
        }
    }

    #[test]
    fn lda_x_params_downcast_succeeds() {
        let p: Box<dyn FunctionalParams> = Box::new(LdaXParams::from_defaults());
        let recovered = p.as_any().downcast_ref::<LdaXParams>();
        assert!(recovered.is_some());
        assert_eq!(recovered.unwrap().alpha, 1.0);
    }

    #[test]
    fn lda_x_params_wrong_type_downcast_returns_none() {
        let p: Box<dyn FunctionalParams> = Box::new(LdaXParams::from_defaults());
        let recovered = p.as_any().downcast_ref::<NoParams>();
        assert!(recovered.is_none());
    }

    #[test]
    fn lda_x_2d_params_defaults_to_one() {
        let p = LdaX2dParams::from_defaults();
        assert_eq!(p.ext_param_count(), 1);
        assert_eq!(p.raw_ext_params(), &[1.0]);
    }

    #[test]
    fn lda_c_pz_no_ext_params() {
        let p = LdaCPzParams;
        assert_eq!(p.ext_param_count(), 0);
        assert!(p.raw_ext_params().is_empty());
    }

    #[test]
    fn lda_c_pz_empty_set_ok() {
        let mut p = LdaCPzParams;
        assert!(p.set_ext_params(&[]).is_ok());
    }

    #[test]
    fn lda_c_pz_nonempty_set_errors() {
        let mut p = LdaCPzParams;
        let r = p.set_ext_params(&[1.0]);
        assert!(matches!(r, Err(LibxcRsError::ExtParamCountMismatch { .. })));
    }
}
