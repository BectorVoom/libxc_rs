//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 938/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk938<F: Float>(t12020: F, t2015: F, t5325: F, t1323: F, t7722: F, t1827: F, t22765: F, t5234: F, t6944: F, t1354: F, t22756: F, t5289: F, t6945: F, t5310: F, t6952: F, t1824: F, t236: F) -> (F, F, F, F, F, F, F, F) {
    let t26225 = t12020 * t2015;
    let t26226 = t26225 * t5325;
    let t26229 = t1323 * t7722;
    let t26231 = t22765 * t1827;
    let t26233 = t5234 * t6944;
    let t26234 = t26233 * t1354;
    let t26236 = t22756 * t1827;
    let t26238 = t6945 * t5289;
    let t26240 = t6952 * t5310;
    let t26243 = t236 * t1824;
    (t26226, t26229, t26231, t26234, t26236, t26238, t26240, t26243)
}
