//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 745/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk745<F: Float>(t187: F, t5343: F, t4433: F, t4436: F, t4439: F, t1625: F, t4528: F, t2281: F, t2285: F, t2310: F, t3182: F, t3183: F, t3189: F, t3194: F, t3196: F, t5326: F, t5327: F) -> (F, F, F, F, F) {
    let t5345 = 0.19751673498613801407e-1 * t5343 * t187;
    let t5346 = 2.0 * t4433;
    let t5347 = 0.36622894612013090108e-3 * t4436;
    let t5348 = 0.11696447245269292414e1 * t4439;
    let t5349 = t4528 * t1625;
    let t5352 = 6.0 * t3183 * t5349 - t2281 - t2285 + t2310 - t3182 + t3189 + t3194 - t3196 - t5326 - t5327 + t5345 + t5346 - t5347 - t5348;
    (t5345, t5346, t5347, t5348, t5352)
}
