//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1073/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1073<F: Float>(t2020: F, t31246: F, t6876: F, t8494: F, t6997: F, t8450: F, t1873: F, t23877: F, t23880: F, t7015: F, t6534: F, t7010: F, t12524: F, t8319: F, t20173: F, t3941: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31247 = t31246 * t2020;
    let t31249 = t6876 * t8494;
    let t31250 = t8450 * t6997;
    let t31270 = t23877 * t1873;
    let t31272 = t23880 * t7015;
    let t31274 = t7010 * t6534;
    let t31277 = 27.0 * t12524 * t8319;
    let t31279 = 27.0 * t20173 * t8319;
    let t31280 = t1873 * t6534;
    let t31282 = 54.0 * t3941 * t31280;
    (t31247, t31249, t31250, t31270, t31272, t31274, t31277, t31279, t31280, t31282)
}
