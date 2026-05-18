//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 748/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk748<F: Float>(t1343: F, t5287: F, t820: F, t1352: F, t5248: F, t5249: F, t120: F, t1799: F, t3805: F, t1831: F, t3866: F, t1307: F) -> (F, F, F, F, F) {
    let t5289 = t1343 * t820 * t5287;
    let t5293 = t5248 * t5249 * t1352;
    let t5301 = t120 * t1799;
    let t5303 = t3805 * t5301 * t1352;
    let t5306 = t3866 * t1831;
    let t5308 = t1799 * t1307;
    (t5289, t5293, t5303, t5306, t5308)
}
