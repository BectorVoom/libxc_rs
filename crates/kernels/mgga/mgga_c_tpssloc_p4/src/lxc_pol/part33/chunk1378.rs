//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1378/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1378<F: Float>(t106956: F, t1873: F, t19451: F, t7467: F, t106878: F, t106881: F, t106921: F, t106923: F, t106932: F, t106934: F, t106937: F, t106939: F, t106941: F, t106953: F, t1458: F, t20347: F, t24999: F, t33085: F, t5493: F, t6517: F, t96686: F) -> F {
    let t106958 = F::cast_from(6.0_f64) * t106956 * t1873;
    let t106960 = F::cast_from(6.0_f64) * t19451 * t7467;
    let t106961 = F::cast_from(6.0_f64) * t1458 * t96686 + F::cast_from(2.0_f64) * t20347 * t6517 + F::cast_from(6.0_f64) * t24999 * t5493 + F::cast_from(6.0_f64) * t33085 * t5493 + t106878 + F::cast_from(6.0_f64) * t106881 + t106921 + t106923 + t106932 + t106934 + t106937 + t106939 + t106941 + t106953 + t106958 + t106960;
    t106961
}
