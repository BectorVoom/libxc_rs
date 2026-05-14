//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1093/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1093<F: Float>(t1874: F, t33234: F, t7042: F, t7461: F, t7685: F, t8641: F, t26193: F, t8621: F, t1985: F, t225: F, t567: F, t7918: F, t214: F, t1842: F, t31558: F, t22635: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33236 = 2.0 * t33234 * t1874;
    let t33238 = 2.0 * t7042 * t7461;
    let t33239 = t7685 * t8641;
    let t33240 = t26193 * t8621;
    let t33241 = t1985 * t33240;
    let t33245 = t7918 * t225 * t567;
    let t33246 = t214 * t33245;
    let t33247 = t1985 * t33246;
    let t33249 = t31558 * t1842;
    let t33250 = t22635 * t33249;
    (t33236, t33238, t33239, t33240, t33241, t33245, t33246, t33247, t33249, t33250)
}
