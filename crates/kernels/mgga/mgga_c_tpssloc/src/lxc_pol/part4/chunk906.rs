//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 906/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk906<F: Float>(t13109: F, t13113: F, t5398: F, t751: F, t707: F, t13133: F, t1462: F, t2427: F, t5597: F, t9922: F, t13124: F, t5522: F, t67: F, t758: F, t3966: F, t4195: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16699 = 0.48830526149350786811e-3 * t13109;
    let t16700 = 0.11696447245269292414e1 * t13113;
    let t16701 = t751 * t5398;
    let t16702 = t707 * t16701;
    let t16703 = 4.0 * t16702;
    let t16705 = 8.0 * t13133 * t1462;
    let t16707 = 4.0 * t2427 * t5597;
    let t16708 = 0.5848223622634646207e0 * t9922;
    let t16709 = 0.21687162600603479684e-1 * t13124;
    let t16710 = t5522 * t67;
    let t16711 = t16710 * t758;
    let t16712 = 0.18311447306006545054e-3 * t16711;
    let t16713 = t4195 * t3966;
    (t16699, t16700, t16703, t16705, t16707, t16708, t16709, t16712, t16713)
}
