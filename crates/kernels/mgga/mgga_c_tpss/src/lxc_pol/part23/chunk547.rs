//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 547/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk547<F: Float>(t45: F, t57: F, t2222: F, t730: F, t200: F, t1985: F, t1992: F, t78: F, t202: F, t81: F, t162: F, t187: F, t150: F, t190: F, t692: F, t725: F, t650: F, t698: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t155 = t57 <= zeta_threshold;
    let t2224 = 0.24415263074675393405e-3 * t730 * t2222;
    let t2225 = 1.0 / t200;
    let t2231 = piecewise3(t151, 0.0, 4.0 / 9.0 * t2225 * t1985 + 4.0 / 3.0 * t78 * t1992);
    let t2232 = 1.0 / t202;
    let t2238 = piecewise3(t155, 0.0, 4.0 / 9.0 * t2232 * t1985 - 4.0 / 3.0 * t81 * t1992);
    let t2239 = t2231 + t2238;
    let t2240 = t2239 * t162;
    let t2242 = 0.19751673498613801407e-1 * t2240 * t187;
    let t2243 = t150 * t2239;
    let t2244 = t2243 * t190;
    let t2245 = t692 * t725;
    let t2246 = 2.0 * t2245;
    let t2250 = t650 * t698;
    (t2224, t2225, t2232, t2239, t2240, t2242, t2243, t2244, t2245, t2246, t2250)
}
