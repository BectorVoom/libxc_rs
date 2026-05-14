//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 366/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk366<F: Float>(t374: F, t376: F, t677: F, t370: F, t121: F, t1013: F, t361: F) -> (F, F, F, F) {
    let t3082 = t374 * t677 * t376;
    let t3084 = t370 * t3082 / 13824.0;
    let t3101 = t121 * t376;
    let t3127 = 1.0 / t1013 / t361;
    (t3082, t3084, t3101, t3127)
}
