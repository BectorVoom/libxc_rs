//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1055/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1055<F: Float>(t11475: F, t11476: F, t3931: F, t11013: F, t3919: F, t11456: F, t11459: F, t11462: F, t11464: F, t11468: F, t2748: F, t3974: F, t3979: F, t8531: F, t8586: F, t925: F, t967: F) -> F {
    let t11477 = t11475 * t11476;
    let t11478 = t3931 * t11477;
    let t11481 = t3919 * t11013;
    let t11486 = -t11456 - t11459 + t11462 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t967 * t11464 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t967 * t11468 + t2748 * t3979 / F::cast_from(216.0_f64) - F::cast_from(5.0_f64) / F::cast_from(1296.0_f64) * t2748 * t3974 - F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t967 * t11478 - t925 * t11481 / F::cast_from(36.0_f64) + F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t8531 + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t8586;
    t11486
}
