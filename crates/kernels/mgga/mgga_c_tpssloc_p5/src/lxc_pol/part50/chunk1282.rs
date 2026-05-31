//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1282/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1282<F: Float>(t31058: F, t4028: F, t1976: F, t26135: F, t652: F, t12725: F, t8327: F, t25010: F, t8450: F, t31051: F, t7458: F, t2314: F, t32663: F) -> (F, F, F, F, F, F) {
    let t120730 = F::cast_from(2.0_f64) * t4028 * t31058;
    let t120732 = t652 * t1976 * t26135;
    let t120735 = F::cast_from(2.0_f64) * t12725 * t8327;
    let t120738 = t8450 * t25010;
    let t120740 = t7458 * t31051;
    let t120742 = t2314 * t32663;
    (t120730, t120732, t120735, t120738, t120740, t120742)
}
