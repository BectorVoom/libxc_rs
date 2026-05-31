//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 388/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk388<F: Float>(t1053: F, t386: F, t68: F, t3032: F, t3127: F, t3031: F, t1932: F, t3131: F, t1014: F, t360: F, t390: F, t1878: F, t268: F, t405: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3173 = F::cast_from(1.0_f64) / t1053 / t386;
    let t3174 = t68 * t3173;
    let t3185 = t3032 * t3127;
    let t3186 = t3031 * t3185;
    let t3188 = t1932 * t3131;
    let t3199 = t3032 * t1014;
    let t3200 = t3031 * t3199;
    let t3201 = t1932 * t360;
    let t3215 = t390 * t390;
    let t3216 = F::cast_from(1.0_f64) / t3215;
    let t3236 = t268 * t1878 * t405;
    (t3173, t3174, t3186, t3188, t3200, t3201, t3215, t3216, t3236)
}
