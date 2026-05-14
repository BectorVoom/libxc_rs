//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1149/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1149<F: Float>(t21128: F, t77: F, t4626: F, t84: F, t4573: F, t578: F, t4580: F, t13298: F, t38: F, t4622: F, t76: F, t1321: F, t1338: F) -> (F, F, F, F, F, F, F, F) {
    let t21129 = t77 * t21128;
    let t21132 = t84 * t4626;
    let t21133 = t77 * t21132;
    let t21136 = t578 * t4573;
    let t21139 = t578 * t4580;
    let t21146 = t13298 * t38;
    let t21165 = t76 * t4622;
    let t21180 = t1321 * t1338;
    (t21129, t21132, t21133, t21136, t21139, t21146, t21165, t21180)
}
