//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1226/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1226<F: Float>(t21132: F, t77: F, t4573: F, t578: F, t4580: F, t13298: F, t38: F, t4622: F, t76: F, t4525: F, t6274: F, t1760: F, t13565: F, t1689: F, t1321: F, t1338: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21133 = t77 * t21132;
    let t21136 = t578 * t4573;
    let t21139 = t578 * t4580;
    let t21146 = t13298 * t38;
    let t21165 = t76 * t4622;
    let t21175 = t6274 * t4525;
    let t21177 = 2.0 * t1760 * t21175;
    let t21179 = 2.0 * t13565 * t1689;
    let t21180 = t1321 * t1338;
    (t21133, t21136, t21139, t21146, t21165, t21175, t21177, t21179, t21180)
}
