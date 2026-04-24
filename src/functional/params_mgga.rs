//! Per-functional `FunctionalParams` impls for MGGA functionals.
//!
//! Plan 05-02 scope: all 86 compiled MGGA functionals + 6 deferred IDs use
//! `NoParams`. The MGGA dispatch helpers in `src/eval/mgga_dispatch/` continue
//! to hardcode the libxc 7.0.0 ext_params defaults at the call site (matching
//! the C oracle). Per-functional concrete `FunctionalParams` impls are
//! tracked as a follow-up plan.

use crate::functional::params::NoParams;

/// Constructor used by `lifecycle.rs::construct_params` for MGGA family IDs.
/// Currently returns `NoParams` for every MGGA functional (Plan 05-02 scope).
#[allow(dead_code)]
pub(crate) fn default_mgga_params() -> NoParams {
    NoParams
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::functional::params::FunctionalParams;

    #[test]
    fn default_mgga_params_is_no_params() {
        let p = default_mgga_params();
        assert_eq!(p.ext_param_count(), 0);
    }
}
