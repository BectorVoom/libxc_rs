//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 767/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk767<F: Float>(t1043: F, t5113: F, t1024: F, t2913: F, t5081: F, t2911: F, t2917: F, t4044: F, t5066: F, t5070: F, t5074: F, t1530: F, t1062: F, t2937: F, t2944: F, t4093: F, t5086: F, t5093: F, t5099: F, t5101: F, t5105: F, t5108: F, t5111: F) -> (F, F, F, F, F, F, F, F) {
    let t5114 = t5113 * t1043;
    let t5116 = 1.0 * t1024 * t5114;
    let t5117 = t5081 * t2913;
    let t5119 = 0.16081979498692535067e2 * t2911 * t5117;
    let t5124 = t2917 - 0.11415555555555555555e-1 * t4044 - 0.11415555555555555555e-1 * t5066 + 0.34246666666666666666e-1 * t5070 + 0.17123333333333333333e-1 * t5074;
    let t5129 = t1530 * t1530;
    let t5130 = t5129 * t1062;
    let t5145 = -0.17648625e1 * t5086 + 0.3529725e1 * t5093 + t2937 - 0.34431666666666666666e0 * t4044 - 0.34431666666666666667e0 * t5066 + 0.103295e1 * t5070 + 0.516475e0 * t5074 + 0.31558125e0 * t5099 + 0.6311625e0 * t5101 + t2944 - 0.13892666666666666667e0 * t4093 - 0.34731666666666666667e-1 * t5105 + 0.20839e0 * t5108 + 0.104195e0 * t5111;
    (t5114, t5116, t5117, t5119, t5124, t5129, t5130, t5145)
}
