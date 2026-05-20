//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1291/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1291<F: Float>(t6888: F, t6891: F, t80707: F, t1377: F, t1385: F, t22633: F, t22635: F, t3719: F, t12033: F, t1386: F, t2016: F, t22630: F, t3752: F, t3758: F, t39916: F, t568: F, t6955: F, t6963: F, t81315: F, t81318: F, t81319: F, t81328: F, t81333: F) -> F {
    let t81339 = t6888 * t80707 * t6891;
    let t81346 = t22633 * t22635 * t1377 * t3719 * t1385;
    let t81348 = F::cast_from(0.49348022005446793095e-1_f64) * t81315 - t81318 - F::new(3.0) * t81319 * t1386 - F::new(3.0) * t39916 * t2016 - F::new(18.0) * t3758 * t22630 - F::cast_from(0.49348022005446793095e-1_f64) * t81328 + F::cast_from(0.14804406601634037928e0_f64) * t81333 + F::new(3.0) * t3752 * t6955 * t568 - F::cast_from(0.49348022005446793095e-1_f64) * t81339 + F::new(6.0) * t12033 * t6963 + F::cast_from(0.49348022005446793095e-1_f64) * t81346;
    t81348
}
