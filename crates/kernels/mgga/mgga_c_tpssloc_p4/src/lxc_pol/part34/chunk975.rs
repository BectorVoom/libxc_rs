//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 975/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk975<F: Float>(t21713: F, t22424: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t1849: F, t19451: F, t20293: F, t20296: F, t20350: F, t20698: F, t20702: F, t20717: F, t20720: F, t4028: F, t510: F, t513: F, t5450: F, t5457: F, t5460: F, t5494: F, t574: F, t6287: F, t6295: F, t6468: F, t652: F, t7458: F) -> (F, F) {
    let t22425 = t21713 + t22424;
    let t22430 = -t113 * t22425 - F::cast_from(3.0_f64) * t1442 * t6287 - F::cast_from(6.0_f64) * t1459 * t19451 - F::cast_from(3.0_f64) * t1774 * t5450 - F::cast_from(6.0_f64) * t1774 * t5457 + F::cast_from(3.0_f64) * t1778 * t6468 + F::cast_from(3.0_f64) * t1849 * t6295 - t20293 * t510 - F::cast_from(6.0_f64) * t20296 * t510 + t20350 * t574 + t20698 * t513 - F::cast_from(6.0_f64) * t20702 * t652 - F::cast_from(6.0_f64) * t20717 * t652 - F::cast_from(2.0_f64) * t20720 * t652 - F::cast_from(12.0_f64) * t4028 * t5460 - F::cast_from(6.0_f64) * t4028 * t5494 - F::cast_from(6.0_f64) * t5494 * t7458;
    (t22425, t22430)
}
