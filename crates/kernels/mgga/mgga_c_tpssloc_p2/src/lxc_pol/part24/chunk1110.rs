//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1110/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1110<F: Float>(t22674: F, t6907: F, t6897: F, t12030: F, t12444: F, t1375: F, t1386: F, t2016: F, t22622: F, t22624: F, t22630: F, t22639: F, t22646: F, t22650: F, t22653: F, t22656: F, t22664: F, t22668: F, t22670: F, t3882: F, t3912: F, t568: F, t6958: F, t6963: F, t6993: F) -> (F, F) {
    let t22675 = t22674 * t6907;
    let t22676 = t6897 * t22675;
    let t22680 = t22622 * t568 + F::cast_from(2.0_f64) * t22624 * t568 + F::cast_from(4.0_f64) * t3882 * t6963 - F::cast_from(6.0_f64) * t1375 * t22630 + F::cast_from(0.3289868133696452873e-1_f64) * t22639 - t22646 + F::cast_from(0.82246703342411321825e-2_f64) * t22650 + F::cast_from(4.0_f64) * t1375 * t22653 - F::cast_from(2.0_f64) * t22656 * t1386 - F::cast_from(2.0_f64) * t12444 * t2016 - t6958 * t3912 - F::cast_from(0.82246703342411321825e-2_f64) * t22664 - F::cast_from(0.16449340668482264365e-1_f64) * t22668 - F::cast_from(2.0_f64) * t22670 * t1386 - t12030 * t2016 + F::cast_from(0.82246703342411321824e-2_f64) * t22676 - F::cast_from(2.0_f64) * t3882 * t6993;
    (t22675, t22680)
}
