//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2016/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2016<F: Float>(t90912: F, t1352: F, t24103: F, t3773: F, t5234: F, t5344: F, t7934: F, t81069: F, t81076: F, t81080: F, t81083: F, t81099: F, t84480: F, t84481: F, t90907: F, t90910: F, t90917: F, t90921: F, t90929: F, t90933: F, t93505: F) -> F {
    let t93572 = F::cast_from(0.15352717957250113407e0_f64) * t90912;
    let t93587 = F::cast_from(0.6579736267392905746e-1_f64) * t90907 + F::cast_from(0.6579736267392905746e-1_f64) * t90910 - t93572 - F::cast_from(0.19739208802178717238e0_f64) * t90917 + F::cast_from(0.9869604401089358619e-1_f64) * t90921 - F::cast_from(0.82246703342411321825e-2_f64) * t81069 - t84480 - t84481 + F::cast_from(0.10417915756705434098e0_f64) * t81076 + t3773 * t7934 - F::cast_from(0.20835831513410868196e0_f64) * t81080 + F::cast_from(0.3289868133696452873e-1_f64) * t81083 + F::cast_from(0.38381794893125283518e-1_f64) * t81099 - F::new(2.0) * t5344 * t93505 * t1352 - t5234 * t24103 - F::cast_from(0.16449340668482264365e-1_f64) * t90929 + F::cast_from(0.3289868133696452873e-1_f64) * t90933;
    t93587
}
