//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 782/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk782<F: Float>(t31366: F, t6555: F, t6552: F, t6572: F, t1880: F, t6547: F, t8557: F, t2047: F, t234: F, t776: F, t6637: F, t794: F, t8556: F, t6562: F, t232: F, t828: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31367 = t31366 * t6555;
    let t31368 = t6552 * t31367;
    let t31370 = t31366 * t6572;
    let t31371 = t1880 * t31370;
    let t31374 = t6547 * t8557;
    let t31376 = t234 * t2047;
    let t31377 = t31376 * t776;
    let t31378 = t6637 * t31377;
    let t31379 = t6552 * t31378;
    let t31381 = t794 * t8556;
    let t31382 = t6562 * t31381;
    let t31385 = t2047 * t828 * t232;
    (t31367, t31368, t31370, t31371, t31374, t31376, t31377, t31378, t31379, t31381, t31382, t31385)
}
