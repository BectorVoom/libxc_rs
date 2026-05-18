//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 954/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk954<F: Float>(t3886: F, t7213: F, t22724: F, t31569: F, t31589: F, t6897: F, t794: F, t22573: F, t8606: F, t32281: F, t580: F, t1404: F, t8811: F) -> (F, F, F, F, F, F) {
    let t115614 = t3886 * t7213;
    let t115629 = t22724 * t31569;
    let t115658 = t6897 * t794 * t31589;
    let t115925 = t8606 * t22573;
    let t116385 = t32281 * t580;
    let t116387 = t8811 * t1404;
    (t115614, t115629, t115658, t115925, t116385, t116387)
}
