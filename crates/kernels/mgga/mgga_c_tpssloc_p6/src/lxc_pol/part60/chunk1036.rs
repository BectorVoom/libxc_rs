//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1036/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1036<F: Float>(t22633: F, t33272: F, t90566: F, t1985: F, t214: F, t225: F, t29286: F, t567: F, t28205: F, t31611: F, t102466: F, t114178: F, t115540: F, t122247: F, t122251: F, t127325: F, t127328: F, t1375: F, t20044: F, t26224: F, t26477: F, t27068: F, t31653: F, t3887: F, t6460: F, t6461: F, t7728: F, t7729: F, t7749: F, t7750: F, t7925: F, t7936: F, t8636: F, t8637: F) -> F {
    let t128740 = t22633 * t90566 * t33272;
    let t128745 = t1985 * t214 * t29286 * t225 * t567;
    let t128758 = t1985 * t31611 * t28205;
    let t128761 = F::new(4.0) * t27068 * t7729 + F::new(2.0) * t1375 * t3887 * t8636 * t6460 + F::new(4.0) * t26477 * t7925 - F::new(2.0) * t27068 * t7750 + F::cast_from(0.3289868133696452873e-1_f64) * t128740 + F::cast_from(0.82246703342411321825e-2_f64) * t128745 + t127325 - F::new(12.0) * t26224 * t102466 * t7728 - t127328 - t20044 * t8637 - t114178 + F::new(4.0) * t1375 * t3887 * t7936 * t7749 - t31653 * t6461 - t115540 + F::cast_from(0.82246703342411321824e-2_f64) * t122247 - F::cast_from(0.82246703342411321825e-2_f64) * t128758 + F::cast_from(0.76763589786250567036e-1_f64) * t122251;
    t128761
}
