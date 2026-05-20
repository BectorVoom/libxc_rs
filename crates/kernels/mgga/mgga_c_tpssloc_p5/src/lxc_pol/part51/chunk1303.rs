//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1303/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1303<F: Float>(t1888: F, t32862: F, t82159: F, t112667: F, t112673: F, t23270: F, t25170: F, t112678: F, t112680: F, t112686: F, t112702: F, t30713: F, t4166: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118498 = F::cast_from(0.3289868133696452873e-1_f64) * t1888 * t82159 * t32862;
    let t118499 = F::cast_from(0.38381794893125283518e-1_f64) * t112667;
    let t118500 = F::cast_from(0.38381794893125283518e-1_f64) * t112673;
    let t118503 = F::cast_from(0.9869604401089358619e-1_f64) * t1888 * t23270 * t25170;
    let t118506 = F::cast_from(0.82246703342411321825e-2_f64) * t112678;
    let t118518 = F::cast_from(0.76763589786250567036e-1_f64) * t112680;
    let t118523 = F::cast_from(0.76763589786250567036e-1_f64) * t112686;
    let t118526 = F::cast_from(0.16449340668482264365e-1_f64) * t112702;
    let t118532 = t4166 * t30713;
    (t118498, t118499, t118500, t118503, t118506, t118518, t118523, t118526, t118532)
}
