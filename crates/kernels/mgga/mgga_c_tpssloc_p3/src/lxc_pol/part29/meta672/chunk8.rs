//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2256/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2256<F: Float>(t22724: F, t26344: F, t22643: F, t7691: F, t81195: F, t1375: F, t16030: F, t16453: F, t1842: F, t2016: F, t22653: F, t22904: F, t26348: F, t3882: F, t3887: F, t5215: F, t539: F, t55093: F, t568: F, t6958: F, t6963: F, t81393: F, t81395: F, t81399: F, t91421: F) -> (F, F) {
    let t91531 = t22724 * t26344;
    let t91548 = t81195 * t22643 * t7691;
    let t91557 = -F::cast_from(2.0_f64) * t55093 * t2016 - F::cast_from(0.38381794893125283518e-1_f64) * t81393 + F::cast_from(2.0_f64) * t1375 * t3887 * t22904 * t1842 + F::cast_from(4.0_f64) * t3882 * t26348 + t539 * t91421 * t568 + F::cast_from(0.16449340668482264365e-1_f64) * t91548 + F::cast_from(4.0_f64) * t16030 * t6963 + F::cast_from(0.38381794893125283518e-1_f64) * t81395 + F::cast_from(4.0_f64) * t6958 * t16453 - t81399 + F::cast_from(4.0_f64) * t5215 * t22653;
    (t91531, t91557)
}
