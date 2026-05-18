//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 858/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk858<F: Float>(t2842: F, t69205: F, t3046: F, t30526: F, t556: F, t13902: F, t1612: F, t11704: F, t13905: F, t1587: F, t1326: F, t13911: F) -> (F, F, F, F, F, F, F) {
    let t75364 = t69205 * t2842;
    let t75367 = t30526 * t3046 * t556;
    let t75369 = t13902 * t1612;
    let t75371 = t13905 * t11704;
    let t75373 = t3046 * t1587;
    let t75374 = t1326 * t75373;
    let t75375 = t13911 * t75374;
    (t75364, t75367, t75369, t75371, t75373, t75374, t75375)
}
