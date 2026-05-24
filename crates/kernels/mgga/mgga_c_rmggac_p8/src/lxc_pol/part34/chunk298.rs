//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 298/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk298<F: Float>(t551: F, t665: F, t558: F, t2295: F, t793: F, t2298: F, t797: F, t2301: F, t305: F, t2068: F, t2353: F, t2073: F, t2356: F) -> (F, F, F, F, F, F, F) {
    let t2376 = t665 * t551;
    let t2379 = t665 * t558;
    let t2382 = t793 * t2295;
    let t2384 = t797 * t2298;
    let t2386 = t305 * t2301;
    let t2388 = t2068 * t2353;
    let t2390 = t2073 * t2356;
    (t2376, t2379, t2382, t2384, t2386, t2388, t2390)
}
