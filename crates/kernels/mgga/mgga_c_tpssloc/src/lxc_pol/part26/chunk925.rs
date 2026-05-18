//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 925/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk925<F: Float>(t10472: F, t10882: F, t10481: F, t360: F, t1021: F, t248: F, t1030: F, t3036: F, t1015: F, t3033: F, t3041: F, t3101: F) -> (F, F, F, F, F) {
    let t10883 = t10472 * t10882;
    let t10884 = t10481 * t360;
    let t10886 = t248 * t1021 * t10884;
    let t10889 = t1030 * t3036;
    let t10890 = t1015 * t10889;
    let t10891 = t3033 * t10890;
    let t10895 = t248 * t3101 * t3041;
    (t10883, t10886, t10889, t10891, t10895)
}
