//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1072/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1072<F: Float>(t31136: F, t31219: F, t533: F, t1390: F, t1983: F, t1873: F, t22461: F, t26103: F, t6517: F, t6534: F, t30991: F, t8601: F, t2314: F, t8326: F, t5113: F, t191: F, t192: F, t6872: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31220 = t31136 + t31219;
    let t31221 = t533 * t31220;
    let t31222 = t31221 * t1390;
    let t31223 = t1983 * t31222;
    let t31227 = t22461 * t1873;
    let t31229 = t26103 * t1873;
    let t31231 = t6517 * t6534;
    let t31233 = 2.0 * t30991;
    let t31235 = 4.0 * t8601 * t6534;
    let t31236 = t2314 * t8326;
    let t31237 = 2.0 * t31236;
    let t31238 = t5113 * t8326;
    let t31239 = 2.0 * t31238;
    let t31246 = t6872 * t191 * t192;
    (t31220, t31221, t31222, t31223, t31227, t31229, t31231, t31233, t31235, t31237, t31239, t31246)
}
