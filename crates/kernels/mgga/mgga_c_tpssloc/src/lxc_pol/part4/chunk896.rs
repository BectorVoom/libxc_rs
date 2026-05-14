//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 896/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk896<F: Float>(t40: F, t52: F, t16549: F, t16554: F, t16558: F, t3966: F, t4080: F, t607: F, t73: F, t5392: F, t9438: F, t2440: F, t5398: F, t4087: F, t76: F, t145: F, t185: F, t5520: F, t751: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t150 = t52 <= zeta_threshold;
    let t16562 = piecewise3(t146, 0.0, -8.0 / 27.0 * t16549 * t607 + 8.0 / 9.0 * t4080 * t3966 + 4.0 / 9.0 * t16554 * t607 + 4.0 / 3.0 * t73 * t16558);
    let t16563 = t9438 * t5392;
    let t16568 = t2440 * t5398;
    let t16574 = piecewise3(t150, 0.0, 8.0 / 27.0 * t16563 * t607 + 8.0 / 9.0 * t4087 * t3966 + 4.0 / 9.0 * t16568 * t607 - 4.0 / 3.0 * t76 * t16558);
    let t16575 = t16562 + t16574;
    let t16576 = t145 * t16575;
    let t16577 = t16576 * t185;
    let t16578 = t5520 * t751;
    (t16575, t16577, t16578)
}
