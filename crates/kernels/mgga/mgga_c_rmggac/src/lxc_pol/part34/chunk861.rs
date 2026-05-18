//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 861/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk861<F: Float>(t325: F, t551: F, t13897: F, t15098: F, t30526: F, t1326: F, t75399: F, t13916: F, t13928: F, t1612: F, t11704: F, t13931: F) -> (F, F, F, F, F, F, F) {
    let t75411 = t551 * t325;
    let t75412 = t75411 * t13897;
    let t75414 = t30526 * t15098;
    let t75416 = t1326 * t75399;
    let t75417 = t13916 * t75416;
    let t75419 = t13928 * t1612;
    let t75421 = t13931 * t11704;
    (t75411, t75412, t75414, t75416, t75417, t75419, t75421)
}
