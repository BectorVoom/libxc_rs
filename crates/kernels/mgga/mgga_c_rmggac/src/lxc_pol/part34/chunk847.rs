//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 847/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk847<F: Float>(t15397: F, t333: F, t14230: F, t14249: F, t2067: F, t352: F, t2078: F, t3369: F, t17881: F, t511: F, t1971: F, t69806: F) -> (F, F, F) {
    let t75171 = t15397 * t333;
    let t75174 = t14230 * t14249 * t2067 * t75171;
    let t75177 = t15397 * t352;
    let t75180 = t14230 * t3369 * t2078 * t75177;
    let t75182 = t511 * t17881;
    let t75183 = t1971 * t75182;
    let t75184 = t69806 * t75183;
    (t75174, t75180, t75184)
}
