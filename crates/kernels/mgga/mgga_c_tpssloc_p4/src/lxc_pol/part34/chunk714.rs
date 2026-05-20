//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 714/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk714<F: Float>(t180: F, t2511: F, t9489: F, t9490: F, t761: F, t116: F, t229: F, t597: F, t60: F, t59: F, t212: F, t2386: F) -> (F, F, F, F, F, F, F) {
    let t9493 = F::new(1.0) / t2511 / t180;
    let t9494 = t9489 * t9490 * t9493;
    let t9496 = F::cast_from(0.10254018858216406658e4_f64) * t761 * t9494;
    let t9523 = t229 * t116;
    let t9533 = F::new(1.0) / t60 / t597;
    let t9534 = t59 * t9533;
    let t9537 = t2386 * t212;
    (t9493, t9494, t9496, t9523, t9533, t9534, t9537)
}
