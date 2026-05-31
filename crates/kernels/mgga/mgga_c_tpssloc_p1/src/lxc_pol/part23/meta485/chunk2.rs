//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1486/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1486<F: Float>(t5: F, t79711: F, t112: F, t113: F, t1442: F, t1459: F, t1774: F, t1778: F, t19451: F, t20347: F, t20698: F, t20702: F, t20717: F, t22425: F, t28002: F, t4028: F, t510: F, t5450: F, t5457: F, t5494: F, t6287: F, t652: F, t67001: F, t7458: F, t77944: F, t79553: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t79712 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t79711);
    let t79713 = t79712 * t112;
    let t79729 = -t113 * (t77944 + t79553) - F::cast_from(8.0_f64) * t652 * t1774 * t20347 - F::cast_from(12.0_f64) * t5457 * t6287 - F::cast_from(4.0_f64) * t1442 * t22425 - F::cast_from(6.0_f64) * t5450 * t6287 - t79713 * t510 - F::cast_from(24.0_f64) * t7458 * t20717 + F::cast_from(4.0_f64) * t1778 * t20698 - F::cast_from(24.0_f64) * t4028 * t20717 - F::cast_from(12.0_f64) * t19451 * t5494 - F::cast_from(8.0_f64) * t67001 * t1459 - F::cast_from(24.0_f64) * t28002 * t5494 - F::cast_from(24.0_f64) * t4028 * t20702;
    (t79713, t79729)
}
