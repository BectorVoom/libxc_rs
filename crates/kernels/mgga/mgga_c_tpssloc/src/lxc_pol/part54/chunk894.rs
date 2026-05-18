//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 894/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk894<F: Float>(t1239: F, t3639: F, t500: F, t1376: F) -> (F, F, F, F) {
    let t11604 = t1239 * t1239;
    let t11605 = F::new(1.0) / t11604;
    let t11947 = F::new(1.0) / t3639 / t500;
    let t12019 = t1376 * t1376;
    let t12020 = F::new(1.0) / t12019;
    (t11605, t11947, t12019, t12020)
}
