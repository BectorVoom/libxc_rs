//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1868/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1868<F: Float>(t1827: F, t22756: F, t5289: F, t6945: F, t5310: F, t6952: F, t1824: F, t236: F, t22705: F, t550: F, t22852: F, t2002: F, t5230: F) -> (F, F, F, F, F, F) {
    let t26236 = t22756 * t1827;
    let t26238 = t6945 * t5289;
    let t26240 = t6952 * t5310;
    let t26243 = t236 * t1824;
    let t26245 = t22705 * t26243 * t550;
    let t26246 = t22852 * t26245;
    let t26248 = t5230 * t2002;
    (t26236, t26238, t26240, t26245, t26246, t26248)
}
