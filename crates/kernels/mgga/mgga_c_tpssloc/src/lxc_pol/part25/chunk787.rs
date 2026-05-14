//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 787/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk787<F: Float>(t360: F, t6739: F, t11047: F, t1057: F, t10960: F, t3120: F, t3188: F, t1059: F, t10471: F, t10474: F, t10470: F, t10482: F, t3127: F, t3131: F, t1049: F, t1060: F) -> (F, F, F, F, F, F, F, F) {
    let t11048 = t6739 * t360;
    let t11049 = t11047 * t11048;
    let t11051 = t10960 * t1057;
    let t11054 = t3188 * t3120;
    let t11055 = t1059 * t11054;
    let t11058 = t10471 * t10474;
    let t11059 = t10470 * t11058;
    let t11060 = t6739 * t10482;
    let t11061 = t11047 * t11060;
    let t11064 = t10471 * t3127;
    let t11065 = t10470 * t11064;
    let t11066 = t6739 * t3131;
    let t11067 = t11047 * t11066;
    let t11077 = t1049 * t3120;
    let t11078 = t11077 * t1060;
    (t11049, t11051, t11055, t11059, t11061, t11065, t11067, t11078)
}
