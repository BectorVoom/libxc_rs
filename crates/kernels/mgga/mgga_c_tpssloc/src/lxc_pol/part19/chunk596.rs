//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 596/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk596<F: Float>(t1020: F, t3103: F, t1041: F, t1046: F, t3039: F, t3043: F, t3048: F, t3054: F, t3057: F, t3064: F, t3070: F, t3073: F, t3078: F, t3084: F, t3089: F, t3092: F, t3094: F, t3098: F, t378: F) -> (F,) {
    let t3104 = t1020 * t3103;
    let t3106 = -t3039 * t3043 / 3072.0 - t3048 * t1046 / 432.0 + t3054 / 3456.0 + t1041 * t3057 / 4608.0 + 5.0 / 13824.0 * t1041 * t3064 + t3070 * t3073 / 2304.0 + t3078 * t378 / 3072.0 - t3084 + 19.0 / 1728.0 * t3089 * t378 - t3092 / 432.0 - t3094 * t378 / 288.0 - t1041 * t3098 / 2304.0 + t3104 / 2304.0;
    (t3106,)
}
