//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 909/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk909<F: Float>(t12461: F, t2018: F, t1388: F, t1845: F, t26161: F, t532: F, t7752: F, t6879: F, t1983: F, t1874: F, t26114: F, t4072: F, t89: F, t6525: F, t7458: F, t22751: F, t7692: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26162 = t2018 * t12461;
    let t26163 = t1845 * t1388;
    let t26164 = t26162 * t26163;
    let t26166 = 2.0 * t26161 * t26164;
    let t26167 = t532 * t7752;
    let t26168 = t26167 * t6879;
    let t26170 = 3.0 * t1983 * t26168;
    let t26178 = 2.0 * t26114 * t1874;
    let t26179 = t89 * t4072;
    let t26181 = 2.0 * t26179 * t1874;
    let t26183 = 2.0 * t7458 * t6525;
    let t26184 = t22751 * t7692;
    (t26162, t26163, t26164, t26166, t26168, t26170, t26178, t26179, t26181, t26183, t26184)
}
