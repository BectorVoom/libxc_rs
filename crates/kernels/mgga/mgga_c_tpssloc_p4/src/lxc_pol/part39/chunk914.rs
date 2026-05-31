//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 914/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk914<F: Float>(t180: F, t2511: F, t9489: F, t9490: F, t761: F, t116: F, t229: F, t212: F, t776: F, t2586: F, t597: F, t60: F) -> (F, F, F, F, F, F) {
    let t9493 = F::cast_from(1.0_f64) / t2511 / t180;
    let t9494 = t9489 * t9490 * t9493;
    let t9496 = F::cast_from(0.10254018858216406658e4_f64) * t761 * t9494;
    let t9523 = t229 * t116;
    let t9524 = t212 * t776;
    let t9525 = t9523 * t9524;
    let t9526 = t2586 * t9525;
    let t9533 = F::cast_from(1.0_f64) / t60 / t597;
    (t9493, t9494, t9496, t9523, t9526, t9533)
}
