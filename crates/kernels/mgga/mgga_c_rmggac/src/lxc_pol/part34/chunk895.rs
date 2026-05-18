//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 895/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk895<F: Float>(t13819: F, t8343: F, t13872: F, t15296: F, t13876: F, t13880: F, t14117: F, t68906: F, t74848: F, t14374: F, t15235: F, t14174: F, t17787: F) -> (F, F, F, F, F, F, F) {
    let t76027 = t13819 * t8343;
    let t76029 = t15296 * t13872;
    let t76031 = t15296 * t13876;
    let t76033 = t15296 * t13880;
    let t76036 = t68906 * t14117 * t74848;
    let t76041 = t14374 * t15235;
    let t76043 = t17787 * t14174;
    (t76027, t76029, t76031, t76033, t76036, t76041, t76043)
}
