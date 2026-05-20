//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 809/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk809<F: Float>(t5310: F, t6952: F, t1824: F, t236: F, t22705: F, t550: F, t22852: F, t2002: F, t5230: F, t559: F, t1358: F, t7715: F) -> (F, F, F, F) {
    let t26240 = t6952 * t5310;
    let t26243 = t236 * t1824;
    let t26245 = t22705 * t26243 * t550;
    let t26246 = t22852 * t26245;
    let t26248 = t5230 * t2002;
    let t26249 = t26248 * t559;
    let t26251 = t7715 * t1358;
    (t26240, t26246, t26249, t26251)
}
