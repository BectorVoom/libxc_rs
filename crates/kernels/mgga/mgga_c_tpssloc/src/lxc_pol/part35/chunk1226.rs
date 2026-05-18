//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1226/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1226<F: Float>(t337: F, t5415: F, t131: F, t475: F, t6218: F, t68: F, t7328: F, t1730: F, t8048: F, t2139: F, t6163: F, t471: F) -> (F, F, F, F, F, F, F) {
    let t29584 = t5415 * t337;
    let t29585 = t29584 * t131;
    let t29593 = t6218 * t68 * t475;
    let t29594 = t7328 * t29593;
    let t29597 = t1730 * t8048;
    let t29600 = t2139 * t6163;
    let t29601 = t471 * t29600;
    (t29584, t29585, t29593, t29594, t29597, t29600, t29601)
}
