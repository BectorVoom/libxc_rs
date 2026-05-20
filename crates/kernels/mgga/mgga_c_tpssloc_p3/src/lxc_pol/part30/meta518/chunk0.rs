//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1849/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1849<F: Float>(t1863: F, t26012: F, t1410: F, t2240: F, t6505: F, t7445: F, t4017: F, t71: F, t12568: F, t33: F, t1409: F, t22502: F, t22505: F, t22510: F, t3961: F, t3966: F, t6500: F) -> (F, F, F, F, F, F, F) {
    let t26013 = t1863 * t26012;
    let t26016 = t2240 * t1410;
    let t26021 = t6505 * t7445;
    let t26024 = t71 * t4017;
    let t26025 = t1863 * t26024;
    let t26028 = t12568 * t33;
    let t26043 = -F::new(20.0) / F::new(9.0) * t22502 * t1409 + F::new(5.0) / F::new(18.0) * t22505 * t3961 + F::new(5.0) / F::new(6.0) * t6500 * t3966 - t22510;
    (t26013, t26016, t26021, t26024, t26025, t26028, t26043)
}
