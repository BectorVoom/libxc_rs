//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1213/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1213<F: Float>(t107802: F, t107822: F, t107842: F, t107860: F, t107268: F, t20044: F, t20060: F, t20594: F, t2085: F, t2092: F, t27068: F, t29299: F, t29311: F, t5215: F, t5321: F, t539: F, t568: F, t6361: F, t6440: F, t74908: F, t7918: F, t7925: F, t97664: F) -> (F, F) {
    let t107862 = t107802 + t107822 + t107842 + t107860;
    let t107875 = F::cast_from(0.9869604401089358619e-1_f64) * t107268 + F::new(6.0) * t27068 * t6440 + F::new(6.0) * t20044 * t7925 - F::new(3.0) * t74908 * t2092 - F::new(18.0) * t5215 * t29299 + t539 * t107862 * t568 - F::cast_from(0.69087230807625510332e0_f64) * t97664 + F::new(3.0) * t6361 * t7918 * t568 + F::new(6.0) * t20060 * t7925 + F::new(12.0) * t5321 * t29311 + t20594 * t2085 * t568;
    (t107862, t107875)
}
