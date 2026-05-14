//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1108/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1108<F: Float>(t2517: F, t2658: F, t5392: F, t16616: F, t2528: F, t212: F, t5544: F, t5527: F, t5555: F, t9541: F, t41008: F, t5550: F, t16783: F, t41196: F, t16791: F, t9546: F) -> (F, F, F, F, F, F, F, F) {
    let t59013 = t2658 * t2517 * t5392;
    let t59028 = t16616 * t2528;
    let t59135 = t212 * t5544;
    let t59162 = t212 * t5527;
    let t59195 = t9541 * t5555;
    let t59204 = t41008 * t5550;
    let t59206 = t41196 * t16783;
    let t59218 = t9546 * t16791;
    (t59013, t59028, t59135, t59162, t59195, t59204, t59206, t59218)
}
