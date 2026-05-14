//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 347/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk347<F: Float>(t1021: F, t1046: F, t1049: F, t1054: F, t1063: F, t1069: F, t1073: F, t1082: F, t294: F, t421: F, t425: F) -> (F, F, F) {
    let t1086 = t294 * (-0.310907e-1 * t1049 * t421 + 1.0 * t1054 * t1063 + t1021 - t1046 - 0.19751673498613801407e-1 * t1069 + 0.5848223622634646207e0 * t1073 * t1082);
    let t1088 = 0.19751673498613801407e-1 * t294 * t1069;
    let t1089 = t294 * t425;
    (t1086, t1088, t1089)
}
