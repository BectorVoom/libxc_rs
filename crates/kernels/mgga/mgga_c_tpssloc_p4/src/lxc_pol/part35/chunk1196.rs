//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1196/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1196<F: Float>(t12021: F, t28223: F, t22933: F, t6439: F, t6889: F, t1985: F, t1375: F, t1843: F, t20060: F, t2016: F, t22924: F, t22926: F, t26366: F, t26475: F, t27067: F, t28193: F, t28196: F, t28201: F, t28207: F, t28211: F, t28214: F, t28220: F, t5321: F, t6440: F, t6958: F, t7729: F, t7750: F) -> (F, F, F, F) {
    let t28224 = t12021 * t28223;
    let t28232 = t22933 * t6439;
    let t28233 = t6889 * t28232;
    let t28234 = t1985 * t28233;
    let t28236 = F::cast_from(0.49348022005446793095e-1_f64) * t28193 - F::cast_from(0.16449340668482264365e-1_f64) * t28196 + F::cast_from(0.82246703342411321825e-2_f64) * t28201 - F::cast_from(2.0_f64) * t26366 * t1843 - F::cast_from(0.82246703342411321825e-2_f64) * t28207 - F::cast_from(0.16449340668482264365e-1_f64) * t28211 - F::cast_from(0.3289868133696452873e-1_f64) * t28214 - t27067 - F::cast_from(2.0_f64) * t5321 * t7750 - F::cast_from(0.82246703342411321824e-2_f64) * t26475 + F::cast_from(4.0_f64) * t1375 * t28220 - F::cast_from(6.0_f64) * t1375 * t28224 + F::cast_from(4.0_f64) * t5321 * t7729 + F::cast_from(2.0_f64) * t6958 * t6440 - t20060 * t2016 + F::cast_from(0.16449340668482264365e-1_f64) * t28234 + t22924 + t22926;
    (t28224, t28232, t28233, t28236)
}
