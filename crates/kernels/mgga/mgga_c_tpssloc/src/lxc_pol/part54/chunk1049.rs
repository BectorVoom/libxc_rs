//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1049/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1049<F: Float>(t31399: F, t858: F, t23204: F, t8547: F, t6562: F, t2053: F, t2718: F, t6662: F, t26728: F, t6631: F, t6571: F, t7106: F, t6553: F, t1880: F, t225: F, t8544: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31400 = t858 * t31399;
    let t31405 = t23204 * t8547;
    let t31406 = t6562 * t31405;
    let t31407 = 0.41123351671205660912e-2 * t31406;
    let t31409 = t2718 * t2053 * t6662;
    let t31416 = t26728 * t6631;
    let t31419 = t6571 * t7106;
    let t31420 = t6553 * t31419;
    let t31421 = t1880 * t31420;
    let t31423 = t8544 * t225;
    (t31400, t31405, t31407, t31409, t31416, t31419, t31420, t31421, t31423)
}
