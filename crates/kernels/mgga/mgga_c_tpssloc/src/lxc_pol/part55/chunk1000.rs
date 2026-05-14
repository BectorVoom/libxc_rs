//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1000/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1000<F: Float>(t1441: F, t8319: F, t510: F, t1774: F, t8320: F, t7468: F, t8526: F, t12571: F, t8301: F, t1437: F, t8307: F, t8513: F, t1409: F, t31011: F, t1433: F, t79: F) -> (F, F, F, F, F, F, F, F, F) {
    let t33094 = t1441 * t8319;
    let t33096 = 2.0 * t33094 * t510;
    let t33098 = 2.0 * t8320 * t1774;
    let t33100 = 4.0 * t8526 * t7468;
    let t33103 = t12571 * t8301;
    let t33106 = t8307 * t1437;
    let t33107 = t8513 * t33106;
    let t33111 = t8513 * t31011 * t1409;
    let t33118 = t79 * t1433;
    (t33094, t33096, t33098, t33100, t33103, t33106, t33107, t33111, t33118)
}
