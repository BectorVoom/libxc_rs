//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 897/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk897<F: Float>(t157: F, t16575: F, t182: F, t12861: F, t4119: F, t4315: F, t5392: F, t751: F, t2658: F, t2523: F, t5527: F, t262: F, t5544: F, t1484: F, t868: F, t5660: F, t870: F) -> (F, F, F, F, F, F, F, F) {
    let t16579 = t16575 * t157;
    let t16581 = 0.19751673498613801407e-1 * t16579 * t182;
    let t16582 = 2.0 * t12861;
    let t16583 = t4315 * t4119;
    let t16586 = t751 * t5392;
    let t16587 = t2658 * t16586;
    let t16588 = 12.0 * t16587;
    let t16589 = t2523 * t5527;
    let t16592 = t262 * t5544;
    let t16596 = t1484 * t868;
    let t16606 = t5660 * t870;
    (t16581, t16582, t16583, t16588, t16589, t16592, t16596, t16606)
}
