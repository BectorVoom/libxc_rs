//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 755/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk755<F: Float>(t1877: F, t25: F, t8366: F, t8370: F, t1945: F, t225: F, t387: F) -> (F, F, F) {
    let t8374 = t1877 * t8366 * t25 / 2.0 - t1877 * t8370 * t25 / 2.0;
    let t8375 = t1945 * t225;
    let t8376 = t8375 * t387;
    (t8374, t8375, t8376)
}
