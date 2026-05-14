//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 779/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk779<F: Float>(t10472: F, t10875: F, t10481: F, t3131: F, t1021: F, t248: F, t1015: F, t10478: F, t360: F, t1030: F, t3036: F, t3033: F, t3041: F, t3101: F, t3039: F, t3108: F, t3113: F) -> (F, F, F, F, F, F, F, F) {
    let t10876 = t10472 * t10875;
    let t10877 = t10481 * t3131;
    let t10879 = t248 * t1021 * t10877;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    let t10884 = t10481 * t360;
    let t10886 = t248 * t1021 * t10884;
    let t10889 = t1030 * t3036;
    let t10890 = t1015 * t10889;
    let t10891 = t3033 * t10890;
    let t10895 = t248 * t3101 * t3041;
    let t10896 = t3039 * t10895;
    let t10898 = t3113 * t3108;
    (t10876, t10879, t10883, t10886, t10889, t10891, t10896, t10898)
}
