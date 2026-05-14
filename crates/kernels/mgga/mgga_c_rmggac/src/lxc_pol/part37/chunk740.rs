//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 740/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk740<F: Float>(t69296: F, t75351: F, t1635: F, t262: F, t25636: F, t3068: F, t1624: F, t3076: F, t2044: F, t25518: F, t556: F, t69199: F, t2842: F, t69205: F, t3046: F, t30526: F) -> (F, F, F, F, F, F, F, F) {
    let t75352 = t69296 * t75351;
    let t75355 = t262 * t1635;
    let t75356 = t25636 * t3068 * t75355;
    let t75359 = t3076 * t1624;
    let t75360 = t25518 * t2044 * t75359;
    let t75362 = t69199 * t556;
    let t75364 = t69205 * t2842;
    let t75367 = t30526 * t3046 * t556;
    (t75352, t75355, t75356, t75359, t75360, t75362, t75364, t75367)
}
