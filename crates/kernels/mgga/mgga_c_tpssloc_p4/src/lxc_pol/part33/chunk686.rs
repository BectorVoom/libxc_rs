//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 686/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk686<F: Float>(t1375: F, t1843: F, t5215: F, t5321: F, t568: F, t6362: F, t6364: F, t6435: F, t6440: F, t6461: F) -> F {
    let t6463 = F::cast_from(2.0_f64) * t1375 * t6440 - t1375 * t6461 - F::cast_from(2.0_f64) * t1843 * t5215 - F::cast_from(2.0_f64) * t1843 * t5321 + t568 * t6362 + F::cast_from(2.0_f64) * t568 * t6364 + t568 * t6435;
    t6463
}
