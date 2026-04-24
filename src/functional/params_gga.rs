//! Per-functional `FunctionalParams` impls for GGA functionals.
//!
//! Plan 05-02 scope: all 106 compiled GGA functionals use `NoParams`. The GGA
//! dispatch helpers in `src/eval/gga_dispatch/` continue to hardcode the
//! libxc 7.0.0 ext_params defaults at the call site (matching the C oracle
//! bit-for-bit). Per-functional concrete `FunctionalParams` impls — including
//! the formula-driven CAM/CAMY/CAMG/LC/LCY derivations — are tracked as a
//! follow-up plan.
//!
//! This file exposes a single `gga_params_for(id)` constructor (delegated
//! from `lifecycle.rs::construct_params`) returning `Box<NoParams>` for
//! every GGA id. When a follow-up plan adds concrete per-functional
//! params, this file is where they land.

use crate::functional::params::NoParams;

/// Constructor used by `lifecycle.rs::construct_params` for GGA family IDs.
/// Currently returns `NoParams` for every GGA functional (Plan 05-02 scope).
#[allow(dead_code)]
pub(crate) fn default_gga_params() -> NoParams {
    NoParams
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functional::params::FunctionalParams;

    #[test]
    fn default_gga_params_is_no_params() {
        let p = default_gga_params();
        assert_eq!(p.ext_param_count(), 0);
    }
}
