//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1192/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1192<F: Float>(t19525: F, t19569: F, t509: F, t1270: F, t1760: F, t13119: F, t1778: F, t196: F, t197: F, t4352: F, t1779: F, t1759: F, t7309: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19570 = t19525 + t19569;
    let t19571 = t509 * t19570;
    let t19572 = t19571 * t1270;
    let t19573 = t1760 * t19572;
    let t19574 = t1778 * t13119;
    let t19575 = t1760 * t19574;
    let t19577 = t4352 * t196 * t197;
    let t19578 = t19577 * t1779;
    let t19579 = t1759 * t7309;
    (t19570, t19571, t19572, t19573, t19574, t19575, t19577, t19578, t19579)
}
