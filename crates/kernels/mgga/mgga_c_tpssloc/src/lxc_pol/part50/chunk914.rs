//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 914/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk914<F: Float>(t22705: F, t26243: F, t550: F, t22852: F, t2002: F, t5230: F, t559: F, t1358: F, t7715: F, t1831: F, t22783: F, t5234: F, t6951: F, t1369: F, t22788: F, t5314: F, t6952: F) -> (F, F, F, F, F, F, F) {
    let t26245 = t22705 * t26243 * t550;
    let t26246 = t22852 * t26245;
    let t26248 = t5230 * t2002;
    let t26249 = t26248 * t559;
    let t26251 = t7715 * t1358;
    let t26255 = t22783 * t1831;
    let t26257 = t5234 * t6951;
    let t26258 = t26257 * t1369;
    let t26260 = t22788 * t1831;
    let t26262 = t6952 * t5314;
    (t26246, t26249, t26251, t26255, t26258, t26260, t26262)
}
