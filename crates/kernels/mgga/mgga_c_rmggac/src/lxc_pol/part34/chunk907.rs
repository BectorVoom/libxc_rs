//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 907/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk907<F: Float>(t2079: F, t262: F, t36: F, t8794: F, t14302: F, t75374: F, t14305: F, t75416: F, t1326: F, t14309: F, t1652: F, t3046: F) -> (F, F, F, F) {
    let t76242 = t2079 * t262 * t36 * t8794;
    let t76244 = t14302 * t75374;
    let t76246 = t14305 * t75416;
    let t76250 = t14309 * t1326 * t3046 * t1652;
    (t76242, t76244, t76246, t76250)
}
