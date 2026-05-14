//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 627/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk627<F: Float>(t26078: F, t3046: F, t3056: F, t71: F, t7311: F, t14063: F, t2190: F, t3151: F, t1327: F, t640: F, t668: F, t7323: F, t3154: F, t7921: F, t14040: F, t14367: F) -> (F, F, F, F, F) {
    let t69819 = t3056 * t3046 * t26078 * t71 * t7311;
    let t69827 = t2190 * t14063 * t3151;
    let t69832 = t7323 * t640 * t668 * t1327;
    let t69835 = 0.66211599834018861287e-4 * t7921 * t3154;
    let t69836 = t14040 * t14367;
    (t69819, t69827, t69832, t69835, t69836)
}
