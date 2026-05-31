//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 888/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk888<F: Float>(t10470: F, t10471: F, t1013: F, t363: F, t3034: F, t6793: F, t368: F, t1022: F, t3040: F, t3131: F, t360: F, t1021: F, t248: F) -> (F, F, F, F, F, F, F, F) {
    let t10472 = t10470 * t10471;
    let t10473 = t1013 * t1013;
    let t10474 = F::cast_from(1.0_f64) / t10473;
    let t10475 = t10474 * t363;
    let t10477 = F::cast_from(1.0_f64) / t3034 / t6793;
    let t10478 = t368 * t10477;
    let t10479 = t10475 * t10478;
    let t10480 = t10472 * t10479;
    let t10481 = t3040 * t1022;
    let t10482 = t3131 * t360;
    let t10483 = t10481 * t10482;
    let t10485 = t248 * t1021 * t10483;
    (t10472, t10474, t10477, t10478, t10480, t10481, t10482, t10485)
}
