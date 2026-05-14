//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 826/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk826<F: Float>(t2091: F, t40590: F, t12020: F, t7213: F, t2098: F, t2319: F, t12734: F, t8327: F, t2314: F, t31058: F, t3652: F, t652: F, t8326: F, t12823: F, t4034: F, t9348: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93319 = t40590 * t2091;
    let t93818 = t12020 * t7213;
    let t94165 = t2098 * t2319;
    let t112521 = 4.0 * t12734 * t8327;
    let t112523 = 4.0 * t2314 * t31058;
    let t112528 = 2.0 * t652 * t3652 * t8326;
    let t112535 = 2.0 * t12823 * t8327;
    let t112537 = 4.0 * t4034 * t31058;
    let t112542 = 2.0 * t9348 * t8327;
    (t93319, t93818, t94165, t112521, t112523, t112528, t112535, t112537, t112542)
}
