//! Per-functional `FunctionalParams` impls for MGGA functionals.
//!
//! Every compiled MGGA functional listed in `src/meta/generated.rs` has a
//! dedicated marker `*Params` struct that implements [`FunctionalParams`].
//! Dispatch arms continue to consume hardcoded libxc 7.0.0 defaults
//! (matching the C oracle) until per-functional ext-params plumbing lands
//! in a follow-up plan.
//!
//! **Scope:** includes markers for the 25 routable `MggaFunctional` variants
//! plus all remaining compiled MGGA functionals (total 86+ matching the
//! libxc 7.0.0 MGGA set). Each marker exists to give `Functional::new` a
//! typed `&dyn FunctionalParams` to hand dispatch, even for functionals
//! whose dispatch arm currently returns `UnsupportedFunctional`.

#![allow(dead_code, non_camel_case_types)]

use std::any::Any;

use libxc_core::error::LibxcRsError;
use crate::functional::params::FunctionalParams;
use libxc_core::model::FunctionalId;

/// Constructor helper used by `lifecycle.rs::construct_params` for MGGA family IDs.
#[allow(dead_code)]
pub(crate) fn default_mgga_params() -> crate::functional::params::NoParams {
    crate::functional::params::NoParams::default()
}

#[cfg(test)]
mod bootstrap_tests {
    use super::*;
    use crate::functional::params::FunctionalParams;

    #[test]
    fn default_mgga_params_is_no_params() {
        let p = default_mgga_params();
        assert_eq!(p.ext_param_count(), 0);
    }
}

/// Zero-ext_params marker for `HybMggaXDldf` (id 36).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXDldfParams;

impl FunctionalParams for HybMggaXDldfParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(36),
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

/// Zero-ext_params marker for `MggaXcZlp` (id 42).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXcZlpParams;

impl FunctionalParams for MggaXcZlpParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(42),
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

/// Zero-ext_params marker for `MggaCCs` (id 72).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCCsParams;

impl FunctionalParams for MggaCCsParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(72),
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

/// Zero-ext_params marker for `MggaXLta` (id 201).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXLtaParams;

impl FunctionalParams for MggaXLtaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(201),
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

/// Zero-ext_params marker for `MggaXTpss` (id 202).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXTpssParams;

impl FunctionalParams for MggaXTpssParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(202),
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

/// Zero-ext_params marker for `MggaXTauHcth` (id 205).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXTauHcthParams;

impl FunctionalParams for MggaXTauHcthParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(205),
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

/// Zero-ext_params marker for `MggaXTb09` (id 208).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXTb09Params;

impl FunctionalParams for MggaXTb09Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(208),
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

/// Zero-ext_params marker for `MggaXPkzb` (id 213).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXPkzbParams;

impl FunctionalParams for MggaXPkzbParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(213),
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

/// Zero-ext_params marker for `MggaXTh` (id 225).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXThParams;

impl FunctionalParams for MggaXThParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(225),
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

/// Zero-ext_params marker for `MggaXcCc06` (id 229).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXcCc06Params;

impl FunctionalParams for MggaXcCc06Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(229),
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

/// Zero-ext_params marker for `MggaXJk` (id 256).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXJkParams;

impl FunctionalParams for MggaXJkParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(256),
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

/// Zero-ext_params marker for `MggaXMvs` (id 257).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMvsParams;

impl FunctionalParams for MggaXMvsParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(257),
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

/// Zero-ext_params marker for `MggaXRtpss` (id 299).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXRtpssParams;

impl FunctionalParams for MggaXRtpssParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(299),
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

/// Zero-ext_params marker for `MggaCCc` (id 387).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCCcParams;

impl FunctionalParams for MggaCCcParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(387),
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

/// Zero-ext_params marker for `HybMggaXM05` (id 438).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXM05Params;

impl FunctionalParams for HybMggaXM05Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(438),
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

/// Zero-ext_params marker for `MggaXTm` (id 540).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXTmParams;

impl FunctionalParams for MggaXTmParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(540),
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

/// Zero-ext_params marker for `MggaXcLp90` (id 564).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXcLp90Params;

impl FunctionalParams for MggaXcLp90Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(564),
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

/// Zero-ext_params marker for `MggaXGx` (id 575).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXGxParams;

impl FunctionalParams for MggaXGxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(575),
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

/// Zero-ext_params marker for `MggaXPbeGx` (id 576).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXPbeGxParams;

impl FunctionalParams for MggaXPbeGxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(576),
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

/// Zero-ext_params marker for `MggaX2dJs17` (id 609).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaX2dJs17Params;

impl FunctionalParams for MggaX2dJs17Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(609),
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

/// Zero-ext_params marker for `MggaKRda` (id 621).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaKRdaParams;

impl FunctionalParams for MggaKRdaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(621),
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

/// Zero-ext_params marker for `MggaKGea2` (id 627).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaKGea2Params;

impl FunctionalParams for MggaKGea2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(627),
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

/// Zero-ext_params marker for `MggaKGea4` (id 628).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaKGea4Params;

impl FunctionalParams for MggaKGea4Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(628),
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

/// Zero-ext_params marker for `MggaXRlda` (id 688).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXRldaParams;

impl FunctionalParams for MggaXRldaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(688),
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

/// Zero-ext_params marker for `MggaXTask` (id 707).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXTaskParams;

impl FunctionalParams for MggaXTaskParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(707),
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

/// Zero-ext_params marker for `MggaCDldf` (id 37).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCDldfParams;

impl FunctionalParams for MggaCDldfParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(37),
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

/// Zero-ext_params marker for `MggaXcOtpssD` (id 64).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXcOtpssDParams;

impl FunctionalParams for MggaXcOtpssDParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(64),
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

/// Zero-ext_params marker for `MggaCMn12Sx` (id 73).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCMn12SxParams;

impl FunctionalParams for MggaCMn12SxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(73),
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

/// Zero-ext_params marker for `MggaCMn12L` (id 74).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCMn12LParams;

impl FunctionalParams for MggaCMn12LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(74),
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

/// Zero-ext_params marker for `MggaCM11L` (id 75).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM11LParams;

impl FunctionalParams for MggaCM11LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(75),
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

/// Zero-ext_params marker for `MggaCM11` (id 76).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM11Params;

impl FunctionalParams for MggaCM11Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(76),
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

/// Zero-ext_params marker for `MggaCM08So` (id 77).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM08SoParams;

impl FunctionalParams for MggaCM08SoParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(77),
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

/// Zero-ext_params marker for `MggaCM08Hx` (id 78).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM08HxParams;

impl FunctionalParams for MggaCM08HxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(78),
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

/// Zero-ext_params marker for `MggaCRevm11` (id 172).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCRevm11Params;

impl FunctionalParams for MggaCRevm11Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(172),
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

/// Zero-ext_params marker for `MggaXM06L` (id 203).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXM06LParams;

impl FunctionalParams for MggaXM06LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(203),
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

/// Zero-ext_params marker for `MggaXGvt4` (id 204).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXGvt4Params;

impl FunctionalParams for MggaXGvt4Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(204),
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

/// Zero-ext_params marker for `MggaXBr89` (id 206).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXBr89Params;

impl FunctionalParams for MggaXBr89Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(206),
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

/// Zero-ext_params marker for `MggaXBj06` (id 207).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXBj06Params;

impl FunctionalParams for MggaXBj06Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(207),
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

/// Zero-ext_params marker for `MggaXRpp09` (id 209).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXRpp09Params;

impl FunctionalParams for MggaXRpp09Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(209),
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

/// Zero-ext_params marker for `MggaX2dPrhg07` (id 210).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaX2dPrhg07Params;

impl FunctionalParams for MggaX2dPrhg07Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(210),
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

/// Zero-ext_params marker for `MggaX2dPrhg07Prp10` (id 211).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaX2dPrhg07Prp10Params;

impl FunctionalParams for MggaX2dPrhg07Prp10Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(211),
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

/// Zero-ext_params marker for `MggaXRevtpss` (id 212).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXRevtpssParams;

impl FunctionalParams for MggaXRevtpssParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(212),
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

/// Zero-ext_params marker for `MggaXBr891` (id 214).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXBr891Params;

impl FunctionalParams for MggaXBr891Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(214),
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

/// Zero-ext_params marker for `MggaKPgsl025` (id 220).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaKPgsl025Params;

impl FunctionalParams for MggaKPgsl025Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(220),
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

/// Zero-ext_params marker for `MggaXMs0` (id 221).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMs0Params;

impl FunctionalParams for MggaXMs0Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(221),
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

/// Zero-ext_params marker for `MggaXMs1` (id 222).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMs1Params;

impl FunctionalParams for MggaXMs1Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(222),
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

/// Zero-ext_params marker for `MggaXMs2` (id 223).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMs2Params;

impl FunctionalParams for MggaXMs2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(223),
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

/// Zero-ext_params marker for `HybMggaXMs2h` (id 224).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXMs2hParams;

impl FunctionalParams for HybMggaXMs2hParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(224),
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

/// Zero-ext_params marker for `MggaXM11L` (id 226).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXM11LParams;

impl FunctionalParams for MggaXM11LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(226),
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

/// Zero-ext_params marker for `MggaXMn12L` (id 227).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMn12LParams;

impl FunctionalParams for MggaXMn12LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(227),
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

/// Zero-ext_params marker for `MggaXMs2Rev` (id 228).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMs2RevParams;

impl FunctionalParams for MggaXMs2RevParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(228),
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

/// Zero-ext_params marker for `MggaXMk00` (id 230).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMk00Params;

impl FunctionalParams for MggaXMk00Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(230),
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

/// Zero-ext_params marker for `MggaCTpss` (id 231).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCTpssParams;

impl FunctionalParams for MggaCTpssParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(231),
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

/// Zero-ext_params marker for `MggaCVsxc` (id 232).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCVsxcParams;

impl FunctionalParams for MggaCVsxcParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(232),
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

/// Zero-ext_params marker for `MggaCM06L` (id 233).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM06LParams;

impl FunctionalParams for MggaCM06LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(233),
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

/// Zero-ext_params marker for `MggaCM06Hf` (id 234).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM06HfParams;

impl FunctionalParams for MggaCM06HfParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(234),
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

/// Zero-ext_params marker for `MggaCM06` (id 235).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM06Params;

impl FunctionalParams for MggaCM06Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(235),
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

/// Zero-ext_params marker for `MggaCM062x` (id 236).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM062xParams;

impl FunctionalParams for MggaCM062xParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(236),
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

/// Zero-ext_params marker for `MggaCM05` (id 237).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM05Params;

impl FunctionalParams for MggaCM05Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(237),
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

/// Zero-ext_params marker for `MggaCM052x` (id 238).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCM052xParams;

impl FunctionalParams for MggaCM052xParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(238),
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

/// Zero-ext_params marker for `MggaCPkzb` (id 239).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCPkzbParams;

impl FunctionalParams for MggaCPkzbParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(239),
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

/// Zero-ext_params marker for `MggaCBc95` (id 240).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCBc95Params;

impl FunctionalParams for MggaCBc95Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(240),
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

/// Zero-ext_params marker for `MggaCRevtpss` (id 241).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCRevtpssParams;

impl FunctionalParams for MggaCRevtpssParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(241),
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

/// Zero-ext_params marker for `MggaXcTpsslyp1w` (id 242).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXcTpsslyp1wParams;

impl FunctionalParams for MggaXcTpsslyp1wParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(242),
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

/// Zero-ext_params marker for `MggaXMk00b` (id 243).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMk00bParams;

impl FunctionalParams for MggaXMk00bParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(243),
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

/// Zero-ext_params marker for `MggaXBloc` (id 244).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXBlocParams;

impl FunctionalParams for MggaXBlocParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(244),
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

/// Zero-ext_params marker for `MggaXModtpss` (id 245).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXModtpssParams;

impl FunctionalParams for MggaXModtpssParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(245),
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

/// Zero-ext_params marker for `MggaCTpssloc` (id 247).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCTpsslocParams;

impl FunctionalParams for MggaCTpsslocParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(247),
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

/// Zero-ext_params marker for `HybMggaXMn12Sx` (id 248).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXMn12SxParams;

impl FunctionalParams for HybMggaXMn12SxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(248),
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

/// Zero-ext_params marker for `MggaXMbeef` (id 249).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMbeefParams;

impl FunctionalParams for MggaXMbeefParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(249),
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

/// Zero-ext_params marker for `MggaXMbeefvdw` (id 250).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMbeefvdwParams;

impl FunctionalParams for MggaXMbeefvdwParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(250),
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

/// Zero-ext_params marker for `MggaCTm` (id 251).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCTmParams;

impl FunctionalParams for MggaCTmParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(251),
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

/// Zero-ext_params marker for `MggaXcB97mV` (id 254).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXcB97mVParams;

impl FunctionalParams for MggaXcB97mVParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(254),
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

/// Zero-ext_params marker for `MggaXMn15L` (id 260).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMn15LParams;

impl FunctionalParams for MggaXMn15LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(260),
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

/// Zero-ext_params marker for `MggaCMn15L` (id 261).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCMn15LParams;

impl FunctionalParams for MggaCMn15LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(261),
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

/// Zero-ext_params marker for `MggaXScan` (id 263).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXScanParams;

impl FunctionalParams for MggaXScanParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(263),
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

/// Zero-ext_params marker for `HybMggaXScan0` (id 264).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXScan0Params;

impl FunctionalParams for HybMggaXScan0Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(264),
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

/// Zero-ext_params marker for `MggaCScan` (id 267).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCScanParams;

impl FunctionalParams for MggaCScanParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(267),
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

/// Zero-ext_params marker for `HybMggaXMn15` (id 268).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXMn15Params;

impl FunctionalParams for HybMggaXMn15Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(268),
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

/// Zero-ext_params marker for `MggaCMn15` (id 269).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCMn15Params;

impl FunctionalParams for MggaCMn15Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(269),
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

/// Zero-ext_params marker for `HybMggaXBmk` (id 279).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXBmkParams;

impl FunctionalParams for HybMggaXBmkParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(279),
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

/// Zero-ext_params marker for `HybMggaXTauHcth` (id 282).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXTauHcthParams;

impl FunctionalParams for HybMggaXTauHcthParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(282),
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

/// Zero-ext_params marker for `MggaXB00` (id 284).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXB00Params;

impl FunctionalParams for MggaXB00Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(284),
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

/// Zero-ext_params marker for `MggaXcHle17` (id 288).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXcHle17Params;

impl FunctionalParams for MggaXcHle17Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(288),
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

/// Zero-ext_params marker for `MggaCScanRvv10` (id 292).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCScanRvv10Params;

impl FunctionalParams for MggaCScanRvv10Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(292),
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

/// Zero-ext_params marker for `MggaXRevm06L` (id 293).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXRevm06LParams;

impl FunctionalParams for MggaXRevm06LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(293),
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

/// Zero-ext_params marker for `MggaCRevm06L` (id 294).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaCRevm06LParams;

impl FunctionalParams for MggaCRevm06LParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(294),
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

/// Zero-ext_params marker for `HybMggaXM08Hx` (id 295).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXM08HxParams;

impl FunctionalParams for HybMggaXM08HxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(295),
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

/// Zero-ext_params marker for `HybMggaXM08So` (id 296).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXM08SoParams;

impl FunctionalParams for HybMggaXM08SoParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(296),
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

/// Zero-ext_params marker for `HybMggaXM11` (id 297).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXM11Params;

impl FunctionalParams for HybMggaXM11Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(297),
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

/// Zero-ext_params marker for `MggaXMs2b` (id 300).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMs2bParams;

impl FunctionalParams for MggaXMs2bParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(300),
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

/// Zero-ext_params marker for `MggaXMs2bs` (id 301).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMs2bsParams;

impl FunctionalParams for MggaXMs2bsParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(301),
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

/// Zero-ext_params marker for `MggaXMvsb` (id 302).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMvsbParams;

impl FunctionalParams for MggaXMvsbParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(302),
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

/// Zero-ext_params marker for `MggaXMvsbs` (id 303).
#[derive(Debug, Default, Clone, Copy)]
pub struct MggaXMvsbsParams;

impl FunctionalParams for MggaXMvsbsParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(303),
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

/// Zero-ext_params marker for `HybMggaXRevm11` (id 304).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybMggaXRevm11Params;

impl FunctionalParams for HybMggaXRevm11Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(304),
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
