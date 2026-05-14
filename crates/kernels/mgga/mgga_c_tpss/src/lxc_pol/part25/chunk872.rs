//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 872/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk872<F: Float>(t2215: F, t3178: F, t2345: F, t3204: F, t540: F, t1183: F, t2331: F, t489: F, t497: F, t7998: F, t19: F, t571: F, t498: F, t1170: F, t3197: F, t1186: F, t3214: F) -> (F, F, F, F, F, F, F, F) {
    let t9887 = t3178 * t2215;
    let t9890 = t3178 * t2345;
    let t9895 = 1.0 / t3204 / t540;
    let t9899 = t1183 * t2331;
    let t9900 = t489 * t9899;
    let t9902 = t497 * t7998;
    let t9903 = t489 * t9902;
    let t9904 = t19 * t571;
    let t9906 = 120.0 * t9904 * t498;
    let t9907 = t1170 * t3197;
    let t9913 = t3214 * t1186;
    (t9887, t9890, t9895, t9900, t9903, t9906, t9907, t9913)
}
