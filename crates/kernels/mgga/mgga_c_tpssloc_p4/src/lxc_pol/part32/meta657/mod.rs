//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta657 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2086;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta657<F: Float>(t87405: F, t87432: F, t87653: F, t87666: F, t87718: F, t87779: F, t87898: F, t87915: F, t90503: F, t90551: F, t90582: F, t90642: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t92677, t92689, t92781, t92794, t92817, t92863, t92954, t92961, t93335, t93368, t93387, t93438) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2086::<F>(t87405, t87432, t87653, t87666, t87718, t87779, t87898, t87915, t90503, t90551, t90582, t90642);
    (t92677, t92689, t92781, t92794, t92817, t92863, t92954, t92961, t93335, t93368, t93387, t93438)
}
