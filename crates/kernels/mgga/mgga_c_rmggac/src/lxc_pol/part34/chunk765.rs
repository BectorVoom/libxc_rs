//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 765/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk765<F: Float>(t13880: F, t15296: F, t14117: F, t68906: F, t74848: F, t14374: F, t15235: F, t14174: F, t17787: F, t15163: F, t34735: F, t14314: F, t551: F, t262: F, t7204: F, t1587: F, t3080: F) -> (F, F, F, F, F, F, F, F, F) {
    let t76033 = t15296 * t13880;
    let t76036 = t68906 * t14117 * t74848;
    let t76041 = t14374 * t15235;
    let t76043 = t17787 * t14174;
    let t76046 = t34735 * t15163;
    let t76048 = t14314 * t551;
    let t76049 = t262 * t76048;
    let t76050 = t7204 * t76049;
    let t76052 = t3080 * t1587;
    (t76033, t76036, t76041, t76043, t76046, t76048, t76049, t76050, t76052)
}
