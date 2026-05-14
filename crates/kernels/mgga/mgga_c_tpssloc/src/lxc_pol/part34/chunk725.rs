//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 725/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk725<F: Float>(t5519: F, t706: F, t5398: F, t751: F, t707: F, t5522: F, t67: F, t758: F, t1509: F, t1519: F, t5550: F, t9573: F, t118: F, t5527: F, t794: F, t9549: F) -> (F, F, F, F, F, F) {
    let t16689 = t706 * t5519;
    let t16701 = t751 * t5398;
    let t16702 = t707 * t16701;
    let t16710 = t5522 * t67;
    let t16711 = t16710 * t758;
    let t16758 = t1519 * t1509;
    let t16769 = t9573 * t5550;
    let t16783 = t118 * t794 * t5527;
    let t16784 = t9549 * t16783;
    (t16689, t16702, t16711, t16758, t16769, t16784)
}
