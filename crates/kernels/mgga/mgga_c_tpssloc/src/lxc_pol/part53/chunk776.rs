//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 776/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk776<F: Float>(t31011: F, t607: F, t8513: F, t641: F, t79: F, t12461: F, t1388: F, t2314: F, t8327: F, t4034: F, t1266: F, t8326: F, t652: F, t1307: F, t3701: F, t6920: F, t8462: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31013 = t8513 * t31011 * t607;
    let t31024 = t8513 * t79 * t641;
    let t31043 = t12461 * t1388;
    let t31054 = t2314 * t8327;
    let t31055 = 2.0 * t31054;
    let t31056 = t4034 * t8327;
    let t31057 = 2.0 * t31056;
    let t31058 = t1266 * t8326;
    let t31059 = t652 * t31058;
    let t31060 = 2.0 * t31059;
    let t31085 = t3701 * t1307;
    let t31153 = t6920 * t8462;
    (t31013, t31024, t31043, t31055, t31057, t31058, t31060, t31085, t31153)
}
