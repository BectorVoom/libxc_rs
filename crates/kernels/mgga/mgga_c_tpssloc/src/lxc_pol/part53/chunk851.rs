//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 851/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk851<F: Float>(t22573: F, t8606: F, t32281: F, t580: F, t1404: F, t8811: F, t32185: F, t532: F, t193: F, t201: F, t8743: F, t2752: F, t32029: F, t8747: F, t10143: F, t40772: F) -> (F, F, F, F, F, F, F, F, F) {
    let t115925 = t8606 * t22573;
    let t116385 = t32281 * t580;
    let t116387 = t8811 * t1404;
    let t116437 = t532 * t32185;
    let t116473 = t193 * t201 * t8743;
    let t116476 = t32029 * t2752;
    let t116481 = t193 * t201 * t8747;
    let t116492 = t8743 * t10143;
    let t116498 = t8747 * t40772;
    (t115925, t116385, t116387, t116437, t116473, t116476, t116481, t116492, t116498)
}
