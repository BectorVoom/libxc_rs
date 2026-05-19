//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 886/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk886<F: Float>(t10336: F, t221: F, t339: F, t1032: F, t3082: F, t2393: F, t374: F, t376: F, t370: F, t3158: F, t964: F, t10335: F) -> (F, F, F, F, F) {
    let t10337 = t221 * t10336;
    let t10339 = F::cast_from(0.3086419753086419753e-3_f64) * t339 * t10337;
    let t10372 = t1032 * t3082;
    let t10375 = t374 * t2393 * t376;
    let t10377 = t370 * t10375 / F::new(10368.0);
    let t10381 = t964 * t3158;
    let t10383 = t221 * t10335;
    (t10339, t10372, t10377, t10381, t10383)
}
