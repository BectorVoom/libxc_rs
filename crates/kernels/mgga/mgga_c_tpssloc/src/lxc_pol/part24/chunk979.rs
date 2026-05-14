//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 979/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk979<F: Float>(t28: F, t1081: F, t3672: F, t11122: F, t12001: F, t12072: F, t3231: F, t517: F, t12070: F, t157: F, t182: F, t1294: F, t9722: F, t172: F, t3681: F, t763: F, t2528: F, t3691: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t12075 = t3672 * t1081;
    let t12081 = piecewise3(t29, 0.0, -8.0 / 27.0 * t12072 * t12001 + 4.0 / 3.0 * t12075 * t3231 + 4.0 / 3.0 * t517 * t11122);
    let t12083 = (t12070 + t12081) * t157;
    let t12085 = 0.19751673498613801407e-1 * t12083 * t182;
    let t12087 = 0.10389515463408878255e3 * t1294 * t9722;
    let t12088 = t3681 * t172;
    let t12089 = t12088 * t763;
    let t12090 = 0.17544670867903938621e1 * t12089;
    let t12091 = t3691 * t2528;
    (t12083, t12085, t12087, t12090, t12091)
}
