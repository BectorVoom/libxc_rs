//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 736/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk736<F: Float>(t12020: F, t2015: F, t1827: F, t22765: F, t5234: F, t6944: F, t1824: F, t236: F, t22705: F, t550: F, t22852: F, t1358: F, t7715: F) -> (F, F, F, F, F) {
    let t26225 = t12020 * t2015;
    let t26231 = t22765 * t1827;
    let t26233 = t5234 * t6944;
    let t26243 = t236 * t1824;
    let t26245 = t22705 * t26243 * t550;
    let t26246 = t22852 * t26245;
    let t26251 = t7715 * t1358;
    (t26225, t26231, t26233, t26246, t26251)
}
