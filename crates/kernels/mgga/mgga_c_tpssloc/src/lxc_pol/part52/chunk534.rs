//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 534/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk534<F: Float>(t1243: F, t3534: F, t3032: F, t3502: F, t3499: F, t1932: F, t3508: F, t1209: F, t500: F, t526: F, t528: F, t118: F, t521: F, t2375: F, t1294: F, t2371: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3604 = t3534 * t1243;
    let t3609 = t3032 * t3502;
    let t3610 = t3499 * t3609;
    let t3612 = t1932 * t3508;
    let t3623 = t3032 * t1209;
    let t3624 = t3499 * t3623;
    let t3639 = t500 * t500;
    let t3640 = 1.0 / t3639;
    let t3664 = 1.0 / t526;
    let t3672 = 1.0 / t528;
    let t3684 = t521 * t118;
    let t3686 = 0.10843581300301739842e-1 * t3684 * t2375;
    let t3688 = 0.11696447245269292414e1 * t1294 * t2371;
    (t3604, t3610, t3612, t3624, t3639, t3640, t3664, t3672, t3686, t3688)
}
