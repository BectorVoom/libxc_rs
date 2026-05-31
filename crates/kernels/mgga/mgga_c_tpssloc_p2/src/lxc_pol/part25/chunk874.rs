//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 874/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk874<F: Float>(t11177: F, t300: F, t1098: F, t3256: F, t1119: F, t3259: F, t3308: F, t1094: F, t3312: F, t3316: F, t3311: F, t419: F) -> (F, F, F, F, F) {
    let t11179 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t11177;
    let t11180 = t3256 * t1098;
    let t11182 = F::cast_from(3.0_f64) * t11180 * t1119;
    let t11184 = F::cast_from(3.0_f64) * t3259 * t3308;
    let t11185 = t1094 * t3312;
    let t11187 = F::cast_from(0.48245938496077605201e2_f64) * t11185 * t3316;
    let t11189 = F::cast_from(1.0_f64) / t3311 / t419;
    (t11179, t11182, t11184, t11187, t11189)
}
