//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3119/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3119<F: Float>(t1164: F, t15133: F, t4874: F, t11433: F, t18910: F, t1695: F, t51810: F, t64482: F, t11126: F, t6098: F, t6102: F, t18785: F, t3400: F) -> (F, F, F, F, F, F) {
    let t64514 = F::cast_from(0.23392894490538584828e1_f64) * t1164 * t4874 * t15133;
    let t64517 = F::cast_from(0.17315859105681463759e2_f64) * t1164 * t18910 * t11433;
    let t64520 = F::cast_from(0.14035736694323150897e2_f64) * t51810 * t1695 * t64482;
    let t64522 = F::cast_from(0.11696447245269292414e1_f64) * t11126 * t6098;
    let t64524 = F::cast_from(0.5848223622634646207e0_f64) * t11126 * t6102;
    let t64525 = t3400 * t18785;
    (t64514, t64517, t64520, t64522, t64524, t64525)
}
