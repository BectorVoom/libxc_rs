//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1155/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1155<F: Float>(t23204: F, t31419: F, t6562: F, t2752: F, t31429: F, t193: F, t201: F, t8565: F, t10143: F, t31650: F, t6883: F, t31608: F, t1377: F, t7213: F, t22716: F, t8622: F) -> (F, F, F, F, F, F, F, F) {
    let t114965 = t6562 * t23204 * t31419;
    let t114992 = t31429 * t2752;
    let t115009 = t193 * t201 * t8565;
    let t115027 = t8565 * t10143;
    let t115292 = t6883 * t31650;
    let t115294 = t6883 * t31608;
    let t115296 = t1377 * t7213;
    let t115305 = t22716 * t8622;
    (t114965, t114992, t115009, t115027, t115292, t115294, t115296, t115305)
}
