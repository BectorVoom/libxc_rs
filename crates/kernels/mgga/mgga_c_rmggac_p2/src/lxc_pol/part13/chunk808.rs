//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 808/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk808<F: Float>(t5016: F, t9005: F, t1276: F, t2338: F, t638: F, t639: F, t574: F, t7215: F, t1656: F, t2164: F, t5280: F, t640: F) -> (F, F, F, F, F) {
    let t38328 = t5016 * t9005;
    let t38336 = t638 * t639 * t2338 * t1276;
    let t38340 = t638 * t639 * t7215 * t574;
    let t38344 = t638 * t639 * t2164 * t1656;
    let t38348 = t638 * t639 * t640 * t5280;
    (t38328, t38336, t38340, t38344, t38348)
}
