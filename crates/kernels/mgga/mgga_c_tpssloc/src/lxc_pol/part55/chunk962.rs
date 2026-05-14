//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 962/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk962<F: Float>(t31136: F, t31219: F, t533: F, t1390: F, t1983: F, t30991: F, t6534: F, t8601: F, t2314: F, t8326: F, t5113: F, t6876: F, t8494: F, t12524: F, t8319: F, t20173: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31220 = t31136 + t31219;
    let t31221 = t533 * t31220;
    let t31222 = t31221 * t1390;
    let t31223 = t1983 * t31222;
    let t31233 = 2.0 * t30991;
    let t31235 = 4.0 * t8601 * t6534;
    let t31236 = t2314 * t8326;
    let t31237 = 2.0 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0 * t31238;
    let t31249 = t6876 * t8494;
    let t31277 = 27.0 * t12524 * t8319;
    let t31279 = 27.0 * t20173 * t8319;
    (t31220, t31221, t31222, t31223, t31233, t31235, t31237, t31239, t31249, t31277, t31279)
}
