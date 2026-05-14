//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 743/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk743<F: Float>(t1649: F, t1877: F, t1915: F, t2522: F, t28: F, t6670: F, t7541: F, t7650: F, t7656: F, t1873: F, t4028: F, t1458: F, t88: F) -> (F, F, F) {
    let t7663 = 3.0 / 2.0 * t2522 * t7650 + t1877 * t7541 * t28 / 2.0 - t1877 * t6670 * t7656 / 2.0 + t1877 * t1915 * t1649 / 2.0;
    let t7675 = 2.0 * t4028 * t1873;
    let t7676 = t88 * t1458;
    (t7663, t7675, t7676)
}
