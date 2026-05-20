//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1294/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1294<F: Float>(t25577: F, t4630: F, t25580: F, t4571: F, t17906: F, t6765: F, t17884: F, t17655: F, t23541: F, t18029: F, t6754: F, t1036: F, t28572: F) -> (F, F, F, F, F, F, F) {
    let t99495 = t25577 * t4630;
    let t99497 = t25580 * t4571;
    let t99501 = t6765 * t17906;
    let t99507 = t6765 * t17884;
    let t99509 = t23541 * t17655;
    let t99539 = t18029 * t6754;
    let t99590 = t28572 * t1036;
    (t99495, t99497, t99501, t99507, t99509, t99539, t99590)
}
