//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 852/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk852<F: Float>(t1326: F, t75298: F, t13937: F, t2048: F, t558: F, t13940: F, t15105: F, t321: F, t25525: F, t333: F, t25529: F, t74967: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t75299 = t1326 * t75298;
    let t75300 = t13937 * t75299;
    let t75302 = t2048 * t558;
    let t75303 = t1326 * t75302;
    let t75304 = t13940 * t75303;
    let t75307 = t15105 * t321;
    let t75308 = t25525 * t75307;
    let t75311 = t15105 * t333;
    let t75312 = t25529 * t75311;
    let t75314 = t25529 * t74967;
    (t75299, t75300, t75302, t75303, t75304, t75307, t75308, t75311, t75312, t75314)
}
