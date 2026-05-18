//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 684/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk684<F: Float>(t13812: F, t7487: F, t2044: F, t35613: F, t2139: F, t1326: F, t13961: F, t14147: F, t20: F, t253: F, t270: F, t641: F) -> (F, F, F, F) {
    let t68796 = t7487 * t13812;
    let t68800 = t2044 * t35613;
    let t68801 = t2139 * t68800;
    let t68808 = t14147 * t1326 * t13961 * t641 * t253 * t20 * t270;
    (t68796, t68800, t68801, t68808)
}
