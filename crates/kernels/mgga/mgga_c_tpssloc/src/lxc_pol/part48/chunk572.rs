//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 572/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk572<F: Float>(t478: F, t7327: F, t1215: F, t68: F, t475: F, t1202: F, t2140: F, t1209: F, t1211: F, t1207: F, t1222: F, t2141: F, t1225: F, t2139: F, t471: F, t1198: F, t1218: F, t1232: F, t2134: F, t2136: F, t488: F, t7309: F, t7310: F, t7315: F, t7316: F, t7321: F, t7326: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7328 = t7327 * t478;
    let t7329 = t1215 * t68;
    let t7330 = t7329 * t475;
    let t7331 = t7328 * t7330;
    let t7334 = t1202 * t2140;
    let t7337 = t1209 * sigma2;
    let t7338 = t7337 * t1211;
    let t7339 = t1207 * t7338;
    let t7343 = t2141 * t1222 / 2304.0;
    let t7344 = t2139 * t1225;
    let t7345 = t471 * t7344;
    let t7348 = t7309 - t7310 * t1198 / 288.0 + t7315 - 0.10093189023535097714e-3 * t7316 * t2136 - 0.10093189023535097714e-3 * t2134 * t7321 + 0.10093189023535097714e-3 * t7326 * t7331 + t7334 * t488 / 1536.0 + t7339 * t1218 / 1536.0 + t7343 - t7345 * t1232 / 2304.0;
    (t7328, t7330, t7331, t7334, t7337, t7338, t7339, t7344, t7345, t7348)
}
