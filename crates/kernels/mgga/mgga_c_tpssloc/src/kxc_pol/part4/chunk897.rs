//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 897/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk897<F: Float>(t10470: F, t11045: F, t10471: F, t10474: F, t10482: F, t6739: F, t3127: F, t3131: F, t3215: F, t390: F, t268: F, t405: F, t6546: F) -> (F, F, F, F, F, F, F) {
    let t11046 = t10470 * t11045;
    let t11058 = t10471 * t10474;
    let t11059 = t10470 * t11058;
    let t11060 = t6739 * t10482;
    let t11064 = t10471 * t3127;
    let t11065 = t10470 * t11064;
    let t11066 = t6739 * t3131;
    let t11094 = F::new(1.0) / t3215 / t390;
    let t11135 = t268 * t6546 * t405;
    (t11046, t11059, t11060, t11065, t11066, t11094, t11135)
}
