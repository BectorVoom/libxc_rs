//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 652/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk652<F: Float>(t1081: F, t1877: F, t1915: F, t2522: F, t28: F, t6666: F, t6670: F, t6841: F, t6848: F, t1873: F, t2314: F, t5113: F, t1268: F, t6534: F, t1271: F, t191: F) -> (F, F, F, F, F) {
    let t6855 = 3.0 / 2.0 * t2522 * t1915 * t6841 + t1877 * t6666 * t28 / 2.0 - t1877 * t6670 * t6848 / 2.0 + t1877 * t1915 * t1081 / 2.0;
    let t6867 = 2.0 * t2314 * t1873;
    let t6869 = 2.0 * t5113 * t1873;
    let t6871 = 2.0 * t1268 * t6534;
    let t6875 = t1271 * t191;
    (t6855, t6867, t6869, t6871, t6875)
}
