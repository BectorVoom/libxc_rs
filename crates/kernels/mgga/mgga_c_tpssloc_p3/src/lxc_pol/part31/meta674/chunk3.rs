//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2039/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2039<F: Float>(t29430: F, t576: F, t1858: F, t7945: F, t29395: F, t580: F, t2098: F, t6483: F, t101021: F, t103073: F, t103088: F, t1396: F, t1398: F, t1852: F, t27286: F, t3: F, t6471: F, t7240: F, t94113: F, t94118: F, t94120: F, t94122: F) -> F {
    let t103091 = t576 * t29430;
    let t103092 = t7945 * t1858;
    let t103098 = t29395 * t580;
    let t103099 = t2098 * t6483;
    let t103102 = t1398 * (t101021 + t103088) + t103091 + F::new(2.0) * t103092 + t6471 * t7240 + t1396 * t29430 + F::new(2.0) * t1852 * t27286 + t94113 + t103098 + t103099 + t94118 + t94120 + t94122 + t3 * t103073 * t580;
    t103102
}
