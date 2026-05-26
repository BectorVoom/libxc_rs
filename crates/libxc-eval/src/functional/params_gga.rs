//! Per-functional `FunctionalParams` impls for GGA functionals.
//!
//! Every routed GGA functional enum variant (`GgaFunctional`) has a dedicated
//! marker `*Params` struct that implements [`FunctionalParams`]. These are
//! zero-ext_params markers: dispatch arms continue to consume hardcoded
//! libxc 7.0.0 defaults (matching the C oracle bit-for-bit).
//!
//! Ext-param-bearing GGAs (CAM/CAMY/CAMG/LC/LCY families, `gga_x_hjs_*`,
//! `gga_x_lcgau`, etc.) are tracked as a follow-up plan. When they arrive,
//! each will replace its marker with a concrete struct carrying the raw
//! slice + derived scalars (alpha/beta/omega etc.), and the corresponding
//! dispatch arm will `as_any().downcast_ref::<{Name}Params>()` to extract
//! them.

#![allow(dead_code)]

use std::any::Any;

use libxc_core::error::LibxcRsError;
use crate::functional::params::FunctionalParams;
use libxc_core::model::FunctionalId;

/// Constructor helper used by `lifecycle.rs::construct_params` for GGA family IDs.
/// Returns a default-constructed marker matching the `FunctionalId`. Caller
/// is responsible for boxing.
#[allow(dead_code)]
pub(crate) fn default_gga_params() -> crate::functional::params::NoParams {
    crate::functional::params::NoParams
}

#[cfg(test)]
mod bootstrap_tests {
    use super::*;
    use crate::functional::params::FunctionalParams;

    #[test]
    fn default_gga_params_is_no_params() {
        let p = default_gga_params();
        assert_eq!(p.ext_param_count(), 0);
    }
}

/// Zero-ext_params marker for `GgaXHcthA` (id 34).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXHcthAParams;

impl FunctionalParams for GgaXHcthAParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(34),
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

/// Zero-ext_params marker for `GgaXEv93` (id 35).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXEv93Params;

impl FunctionalParams for GgaXEv93Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(35),
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

/// Zero-ext_params marker for `GgaXQ2d` (id 48).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXQ2dParams;

impl FunctionalParams for GgaXQ2dParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(48),
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

/// Zero-ext_params marker for `GgaKTflw` (id 52).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKTflwParams;

impl FunctionalParams for GgaKTflwParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(52),
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

/// Zero-ext_params marker for `GgaKApbeint` (id 54).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKApbeintParams;

impl FunctionalParams for GgaKApbeintParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(54),
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

/// Zero-ext_params marker for `GgaXAk13` (id 56).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXAk13Params;

impl FunctionalParams for GgaXAk13Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(56),
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

/// Zero-ext_params marker for `GgaKMeyer` (id 57).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKMeyerParams;

impl FunctionalParams for GgaKMeyerParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(57),
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

/// Zero-ext_params marker for `GgaXLvRpw86` (id 58).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXLvRpw86Params;

impl FunctionalParams for GgaXLvRpw86Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(58),
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

/// Zero-ext_params marker for `GgaXPbeint` (id 60).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPbeintParams;

impl FunctionalParams for GgaXPbeintParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(60),
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

/// Zero-ext_params marker for `GgaXVmt84` (id 68).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXVmt84Params;

impl FunctionalParams for GgaXVmt84Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(68),
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

/// Zero-ext_params marker for `GgaXVmt` (id 70).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXVmtParams;

impl FunctionalParams for GgaXVmtParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(70),
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

/// Zero-ext_params marker for `GgaXN12` (id 82).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXN12Params;

impl FunctionalParams for GgaXN12Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(82),
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

/// Zero-ext_params marker for `GgaCOpXalpha` (id 84).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCOpXalphaParams;

impl FunctionalParams for GgaCOpXalphaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(84),
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

/// Zero-ext_params marker for `GgaCOpG96` (id 85).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCOpG96Params;

impl FunctionalParams for GgaCOpG96Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(85),
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

/// Zero-ext_params marker for `GgaCOpPbe` (id 86).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCOpPbeParams;

impl FunctionalParams for GgaCOpPbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(86),
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

/// Zero-ext_params marker for `GgaCOpB88` (id 87).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCOpB88Params;

impl FunctionalParams for GgaCOpB88Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(87),
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

/// Zero-ext_params marker for `GgaXSsbSw` (id 90).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXSsbSwParams;

impl FunctionalParams for GgaXSsbSwParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(90),
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

/// Zero-ext_params marker for `GgaXBpccac` (id 98).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXBpccacParams;

impl FunctionalParams for GgaXBpccacParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(98),
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

/// Zero-ext_params marker for `GgaCTca` (id 100).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCTcaParams;

impl FunctionalParams for GgaCTcaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(100),
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

/// Zero-ext_params marker for `GgaXPbe` (id 101).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPbeParams;

impl FunctionalParams for GgaXPbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(101),
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

/// Zero-ext_params marker for `GgaXB86` (id 103).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXB86Params;

impl FunctionalParams for GgaXB86Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(103),
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

/// Zero-ext_params marker for `GgaXB88` (id 106).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXB88Params;

impl FunctionalParams for GgaXB88Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(106),
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

/// Zero-ext_params marker for `GgaXG96` (id 107).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXG96Params;

impl FunctionalParams for GgaXG96Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(107),
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

/// Zero-ext_params marker for `GgaXPw86` (id 108).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPw86Params;

impl FunctionalParams for GgaXPw86Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(108),
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

/// Zero-ext_params marker for `GgaXPw91` (id 109).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPw91Params;

impl FunctionalParams for GgaXPw91Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(109),
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

/// Zero-ext_params marker for `GgaXOptx` (id 110).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXOptxParams;

impl FunctionalParams for GgaXOptxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(110),
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

/// Zero-ext_params marker for `GgaXDk87` (id 111).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXDk87Params;

impl FunctionalParams for GgaXDk87Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(111),
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

/// Zero-ext_params marker for `GgaXLg93` (id 113).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXLg93Params;

impl FunctionalParams for GgaXLg93Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(113),
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

/// Zero-ext_params marker for `GgaXRpbe` (id 117).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXRpbeParams;

impl FunctionalParams for GgaXRpbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(117),
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

/// Zero-ext_params marker for `GgaXWc` (id 118).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXWcParams;

impl FunctionalParams for GgaXWcParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(118),
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

/// Zero-ext_params marker for `GgaXAm05` (id 120).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXAm05Params;

impl FunctionalParams for GgaXAm05Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(120),
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

/// Zero-ext_params marker for `GgaXPbea` (id 121).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPbeaParams;

impl FunctionalParams for GgaXPbeaParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(121),
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

/// Zero-ext_params marker for `GgaXMpbe` (id 122).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXMpbeParams;

impl FunctionalParams for GgaXMpbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(122),
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

/// Zero-ext_params marker for `GgaX2dB86Mgc` (id 124).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaX2dB86MgcParams;

impl FunctionalParams for GgaX2dB86MgcParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(124),
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

/// Zero-ext_params marker for `GgaXBayesian` (id 125).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXBayesianParams;

impl FunctionalParams for GgaXBayesianParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(125),
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

/// Zero-ext_params marker for `GgaX2dB88` (id 127).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaX2dB88Params;

impl FunctionalParams for GgaX2dB88Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(127),
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

/// Zero-ext_params marker for `GgaX2dB86` (id 128).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaX2dB86Params;

impl FunctionalParams for GgaX2dB86Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(128),
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

/// Zero-ext_params marker for `GgaX2dPbe` (id 129).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaX2dPbeParams;

impl FunctionalParams for GgaX2dPbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(129),
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

/// Zero-ext_params marker for `GgaCPbe` (id 130).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCPbeParams;

impl FunctionalParams for GgaCPbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(130),
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

/// Zero-ext_params marker for `GgaCLyp` (id 131).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCLypParams;

impl FunctionalParams for GgaCLypParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(131),
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

/// Zero-ext_params marker for `GgaCP86` (id 132).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCP86Params;

impl FunctionalParams for GgaCP86Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(132),
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

/// Zero-ext_params marker for `GgaCAm05` (id 135).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCAm05Params;

impl FunctionalParams for GgaCAm05Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(135),
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

/// Zero-ext_params marker for `GgaCLm` (id 137).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCLmParams;

impl FunctionalParams for GgaCLmParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(137),
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

/// Zero-ext_params marker for `GgaXRge2` (id 142).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXRge2Params;

impl FunctionalParams for GgaXRge2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(142),
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

/// Zero-ext_params marker for `GgaXKt` (id 145).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXKtParams;

impl FunctionalParams for GgaXKtParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(145),
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

/// Zero-ext_params marker for `GgaCWl` (id 147).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCWlParams;

impl FunctionalParams for GgaCWlParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(147),
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

/// Zero-ext_params marker for `GgaCWi` (id 148).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCWiParams;

impl FunctionalParams for GgaCWiParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(148),
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

/// Zero-ext_params marker for `GgaXSogga11` (id 151).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXSogga11Params;

impl FunctionalParams for GgaXSogga11Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(151),
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

/// Zero-ext_params marker for `GgaXcTh1` (id 154).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXcTh1Params;

impl FunctionalParams for GgaXcTh1Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(154),
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

/// Zero-ext_params marker for `GgaXcTh2` (id 155).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXcTh2Params;

impl FunctionalParams for GgaXcTh2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(155),
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

/// Zero-ext_params marker for `GgaXcTh3` (id 156).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXcTh3Params;

impl FunctionalParams for GgaXcTh3Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(156),
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

/// Zero-ext_params marker for `GgaXC09x` (id 158).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXC09xParams;

impl FunctionalParams for GgaXC09xParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(158),
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

/// Zero-ext_params marker for `GgaXLb` (id 160).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXLbParams;

impl FunctionalParams for GgaXLbParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(160),
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

/// Zero-ext_params marker for `GgaXLspbe` (id 168).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXLspbeParams;

impl FunctionalParams for GgaXLspbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(168),
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

/// Zero-ext_params marker for `GgaXLsrpbe` (id 169).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXLsrpbeParams;

impl FunctionalParams for GgaXLsrpbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(169),
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

/// Zero-ext_params marker for `GgaXNcap` (id 180).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXNcapParams;

impl FunctionalParams for GgaXNcapParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(180),
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

/// Zero-ext_params marker for `GgaXOl2` (id 183).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXOl2Params;

impl FunctionalParams for GgaXOl2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(183),
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

/// Zero-ext_params marker for `GgaKApbe` (id 185).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKApbeParams;

impl FunctionalParams for GgaKApbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(185),
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

/// Zero-ext_params marker for `GgaXHtbs` (id 191).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXHtbsParams;

impl FunctionalParams for GgaXHtbsParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(191),
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

/// Zero-ext_params marker for `GgaXAiry` (id 192).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXAiryParams;

impl FunctionalParams for GgaXAiryParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(192),
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

/// Zero-ext_params marker for `GgaXLag` (id 193).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXLagParams;

impl FunctionalParams for GgaXLagParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(193),
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

/// Zero-ext_params marker for `GgaCPbeVwn` (id 216).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCPbeVwnParams;

impl FunctionalParams for GgaCPbeVwnParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(216),
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

/// Zero-ext_params marker for `GgaKRationalP` (id 218).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKRationalPParams;

impl FunctionalParams for GgaKRationalPParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(218),
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

/// Zero-ext_params marker for `GgaKPg` (id 219).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKPgParams;

impl FunctionalParams for GgaKPgParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(219),
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

/// Zero-ext_params marker for `GgaCP86vwn` (id 252).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCP86vwnParams;

impl FunctionalParams for GgaCP86vwnParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(252),
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

/// Zero-ext_params marker for `GgaCOpPw91` (id 262).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCOpPw91Params;

impl FunctionalParams for GgaCOpPw91Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(262),
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

/// Zero-ext_params marker for `GgaXCap` (id 270).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXCapParams;

impl FunctionalParams for GgaXCapParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(270),
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

/// Zero-ext_params marker for `GgaCBmk` (id 280).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCBmkParams;

impl FunctionalParams for GgaCBmkParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(280),
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

/// Zero-ext_params marker for `GgaXBeefvdw` (id 285).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXBeefvdwParams;

impl FunctionalParams for GgaXBeefvdwParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(285),
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

/// Zero-ext_params marker for `GgaXPbetrans` (id 291).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPbetransParams;

impl FunctionalParams for GgaXPbetransParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(291),
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

/// Zero-ext_params marker for `GgaXChachiyo` (id 298).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXChachiyoParams;

impl FunctionalParams for GgaXChachiyoParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(298),
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

/// Zero-ext_params marker for `GgaCChachiyo` (id 309).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCChachiyoParams;

impl FunctionalParams for GgaCChachiyoParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(309),
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

/// Zero-ext_params marker for `GgaCCcdf` (id 313).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCCcdfParams;

impl FunctionalParams for GgaCCcdfParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(313),
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

/// Zero-ext_params marker for `HybGgaXcCase21` (id 390).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybGgaXcCase21Params;

impl FunctionalParams for HybGgaXcCase21Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(390),
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

/// Zero-ext_params marker for `GgaXS12` (id 495).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXS12Params;

impl FunctionalParams for GgaXS12Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(495),
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

/// Zero-ext_params marker for `GgaKPearson` (id 511).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKPearsonParams;

impl FunctionalParams for GgaKPearsonParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(511),
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

/// Zero-ext_params marker for `GgaKOl1` (id 512).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKOl1Params;

impl FunctionalParams for GgaKOl1Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(512),
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

/// Zero-ext_params marker for `GgaKOl2` (id 513).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKOl2Params;

impl FunctionalParams for GgaKOl2Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(513),
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

/// Zero-ext_params marker for `GgaKPw86` (id 515).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKPw86Params;

impl FunctionalParams for GgaKPw86Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(515),
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

/// Zero-ext_params marker for `GgaKDk` (id 516).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKDkParams;

impl FunctionalParams for GgaKDkParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(516),
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

/// Zero-ext_params marker for `GgaKLc94` (id 521).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKLc94Params;

impl FunctionalParams for GgaKLc94Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(521),
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

/// Zero-ext_params marker for `GgaKLlp` (id 522).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKLlpParams;

impl FunctionalParams for GgaKLlpParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(522),
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

/// Zero-ext_params marker for `GgaKThakkar` (id 523).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKThakkarParams;

impl FunctionalParams for GgaKThakkarParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(523),
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

/// Zero-ext_params marker for `GgaXItyh` (id 529).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXItyhParams;

impl FunctionalParams for GgaXItyhParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(529),
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

/// Zero-ext_params marker for `GgaXSfat` (id 530).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXSfatParams;

impl FunctionalParams for GgaXSfatParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(530),
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

/// Zero-ext_params marker for `GgaXSg4` (id 533).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXSg4Params;

impl FunctionalParams for GgaXSg4Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(533),
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

/// Zero-ext_params marker for `GgaXGg99` (id 535).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXGg99Params;

impl FunctionalParams for GgaXGg99Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(535),
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

/// Zero-ext_params marker for `GgaXPbepow` (id 539).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPbepowParams;

impl FunctionalParams for GgaXPbepowParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(539),
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

/// Zero-ext_params marker for `GgaCScanE0` (id 553).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCScanE0Params;

impl FunctionalParams for GgaCScanE0Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(553),
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

/// Zero-ext_params marker for `GgaCW94` (id 561).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCW94Params;

impl FunctionalParams for GgaCW94Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(561),
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

/// Zero-ext_params marker for `GgaCCs1` (id 565).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCCs1Params;

impl FunctionalParams for GgaCCs1Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(565),
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

/// Zero-ext_params marker for `GgaKExp4` (id 597).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKExp4Params;

impl FunctionalParams for GgaKExp4Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(597),
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

/// Zero-ext_params marker for `GgaXSfatPbe` (id 601).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXSfatPbeParams;

impl FunctionalParams for GgaXSfatPbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(601),
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

/// Zero-ext_params marker for `GgaXFdLb94` (id 604).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXFdLb94Params;

impl FunctionalParams for GgaXFdLb94Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(604),
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

/// Zero-ext_params marker for `GgaKLkt` (id 613).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKLktParams;

impl FunctionalParams for GgaKLktParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(613),
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

/// Zero-ext_params marker for `GgaKMpbe` (id 616).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKMpbeParams;

impl FunctionalParams for GgaKMpbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(616),
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

/// Zero-ext_params marker for `GgaKVt84f` (id 619).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKVt84fParams;

impl FunctionalParams for GgaKVt84fParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(619),
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

/// Zero-ext_params marker for `GgaKLgap` (id 620).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKLgapParams;

impl FunctionalParams for GgaKLgapParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(620),
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

/// Zero-ext_params marker for `GgaXItyhOptx` (id 622).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXItyhOptxParams;

impl FunctionalParams for GgaXItyhOptxParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(622),
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

/// Zero-ext_params marker for `GgaXItyhPbe` (id 623).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXItyhPbeParams;

impl FunctionalParams for GgaXItyhPbeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(623),
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

/// Zero-ext_params marker for `GgaCLypr` (id 624).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaCLyprParams;

impl FunctionalParams for GgaCLyprParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(624),
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

/// Zero-ext_params marker for `GgaKLgapGe` (id 633).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaKLgapGeParams;

impl FunctionalParams for GgaKLgapGeParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(633),
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

/// Zero-ext_params marker for `HybGgaXCamS12` (id 646).
#[derive(Debug, Default, Clone, Copy)]
pub struct HybGgaXCamS12Params;

impl FunctionalParams for HybGgaXCamS12Params {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(646),
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

/// Zero-ext_params marker for `GgaXPbeErfGws` (id 655).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXPbeErfGwsParams;

impl FunctionalParams for GgaXPbeErfGwsParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(655),
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

/// Zero-ext_params marker for `GgaXQ1d` (id 734).
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaXQ1dParams;

impl FunctionalParams for GgaXQ1dParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(734),
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

/// Fallback marker for ext-param-bearing GGAs not yet routed
/// (e.g. `gga_x_hjs_*`, `gga_x_lcgau`, CAM families). Construction is
/// valid but dispatch will return `UnsupportedFunctional`.
#[derive(Debug, Default, Clone, Copy)]
pub struct GgaExtParamFallbackParams;

impl FunctionalParams for GgaExtParamFallbackParams {
    fn ext_param_count(&self) -> usize {
        0
    }

    fn raw_ext_params(&self) -> &[f64] {
        &[]
    }

    fn set_ext_params(&mut self, vals: &[f64]) -> Result<(), LibxcRsError> {
        if !vals.is_empty() {
            return Err(LibxcRsError::ExtParamCountMismatch {
                id: FunctionalId(0),
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
