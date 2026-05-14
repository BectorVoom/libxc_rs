//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 900/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk900<F: Float>(t40: F, t16630: F, t4202: F, t4205: F, t16558: F, t185: F, t707: F, t5392: F, t634: F, t5398: F, t75: F, t3966: F, t4104: F, t607: F, t767: F, t638: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t16631 = 8.0 * t16630;
    let t16633 = 8.0 * t4205 * t4202;
    let t16634 = t185 * t16558;
    let t16636 = 4.0 * t707 * t16634;
    let t16637 = t634 * t5392;
    let t16642 = t75 * t5398;
    let t16648 = piecewise3(t146, 0.0, 8.0 / 27.0 * t16637 * t607 - 4.0 / 9.0 * t4104 * t3966 - 2.0 / 9.0 * t16642 * t607 + 2.0 / 3.0 * t767 * t16558);
    let t16649 = t638 * t5392;
    let t16654 = t78 * t5398;
    (t16631, t16633, t16636, t16648, t16649, t16654)
}
