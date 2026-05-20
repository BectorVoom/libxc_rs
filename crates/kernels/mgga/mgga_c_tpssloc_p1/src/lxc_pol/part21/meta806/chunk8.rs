//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2806/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2806<F: Float>(t16853: F, t2697: F, t16673: F, t2638: F, t831: F, t2693: F, t5576: F, t16965: F, t9573: F, t16997: F, t838: F, t16961: F) -> (F, F, F, F, F, F) {
    let t59279 = t2697 * t16853;
    let t59281 = t16673 * t2638;
    let t59282 = t59281 * t831;
    let t59288 = t5576 * t2693;
    let t59298 = t9573 * t16965;
    let t59308 = t16997 * t838;
    let t59310 = t9573 * t16961;
    (t59279, t59282, t59288, t59298, t59308, t59310)
}
