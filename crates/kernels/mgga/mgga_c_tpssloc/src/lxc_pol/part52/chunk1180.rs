//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1180/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1180<F: Float>(t120120: F, t26114: F, t8326: F, t26117: F, t31717: F, t7467: F, t26135: F, t8601: F, t12725: F, t26103: F, t6517: F, t33211: F, t6534: F, t31537: F, t1873: F, t96361: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t120121 = 2.0 * t120120;
    let t120122 = t26114 * t8326;
    let t120123 = 2.0 * t120122;
    let t120124 = t26117 * t8326;
    let t120125 = 2.0 * t120124;
    let t120127 = 4.0 * t31717 * t7467;
    let t120129 = 4.0 * t8601 * t26135;
    let t120130 = t12725 * t8326;
    let t120131 = 2.0 * t120130;
    let t120132 = t26103 * t7467;
    let t120134 = t6517 * t26135;
    let t120137 = 4.0 * t33211 * t6534;
    let t120140 = 4.0 * t31537 * t7467;
    let t120141 = t96361 * t1873;
    (t120121, t120123, t120125, t120127, t120129, t120131, t120132, t120134, t120137, t120140, t120141)
}
