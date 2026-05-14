//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1080/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1080<F: Float>(t4068: F, t9507: F, t4109: F, t9504: F, t1042: F, t4105: F, t2862: F, t1519: F, t2905: F, t2863: F, t4108: F, t9292: F, t1073: F, t12083: F, t12086: F, t12136: F, t12155: F, t1543: F, t2932: F, t2969: F, t2976: F, t4125: F, t4147: F, t4181: F, t421: F, t9365: F, t9419: F, t9471: F) -> (F, F, F, F, F, F) {
    let t12159 = 4.0 * t9507 * t4068;
    let t12161 = 0.32163958997385070134e2 * t9504 * t4109;
    let t12162 = t4105 * t1042;
    let t12164 = 4.0 * t2862 * t12162;
    let t12165 = t1519 * t2905;
    let t12167 = 2.0 * t2862 * t12165;
    let t12168 = t4108 * t2863;
    let t12170 = 0.96491876992155210402e2 * t9292 * t12168;
    let t12175 = 0.5848223622634646207e0 * t9365 * t1543 + 0.11696447245269292414e1 * t2969 * t4181 - 2.0 * t12083 * t2932 - 0.11696447245269292414e1 * t12086 * t2976 + 0.5848223622634646207e0 * t1073 * t12136 - 0.310907e-1 * t12155 * t421 + t12159 - t12161 + t12164 + t12167 + t12170 - 4.0 * t9471 * t4125 + 0.64327917994770140268e2 * t9419 * t4147;
    (t12159, t12161, t12164, t12167, t12170, t12175)
}
