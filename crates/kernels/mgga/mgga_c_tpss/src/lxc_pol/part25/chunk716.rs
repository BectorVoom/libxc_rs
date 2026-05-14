//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 716/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk716<F: Float>(t4907: F, t885: F, t2577: F, t4891: F, t2581: F, t3746: F, t4828: F, t4832: F, t4836: F, t318: F, t1448: F) -> (F, F, F, F, F) {
    let t4908 = t4907 * t885;
    let t4911 = t4891 * t2577;
    let t4918 = t2581 + 0.61805555555555555556e-2 * t3746 - 0.61805555555555555555e-2 * t4828 + 0.18541666666666666667e-1 * t4832 - 0.92708333333333333333e-2 * t4836;
    let t4919 = t4918 * t318;
    let t4923 = t1448 * t1448;
    (t4908, t4911, t4918, t4919, t4923)
}
