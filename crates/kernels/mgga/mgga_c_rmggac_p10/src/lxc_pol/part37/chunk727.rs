//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 727/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk727<F: Float>(t14148: F, t14150: F, t35718: F, t240: F, t356: F, t4738: F, t7351: F, t14107: F, t6477: F, t14207: F, t2604: F, t1966: F, t1968: F, t68889: F) -> (F, F, F, F, F) {
    let t70517 = t14148 * t35718 * t14150;
    let t70518 = F::cast_from(0.65053455985619242964e-5_f64) * t70517;
    let t70524 = t14148 * t7351 * t356 * t240 * t4738;
    let t70525 = F::cast_from(0.65053455985619242964e-5_f64) * t70524;
    let t70526 = t6477 * t14107;
    let t70545 = t2604 * t14207;
    let t70548 = t1966 * t68889 * t1968;
    (t70518, t70525, t70526, t70545, t70548)
}
