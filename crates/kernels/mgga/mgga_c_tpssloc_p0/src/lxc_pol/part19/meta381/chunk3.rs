//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1426/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1426<F: Float>(t1099: F, t1118: F, t44021: F, t44036: F, t44052: F, t44067: F, t3311: F, t409: F, t3314: F, t43970: F, t11185: F, t11427: F) -> (F, F, F) {
    let t44072 = F::cast_from(1.0_f64) * t1099 * (t44021 + t44036 + t44052 + t44067) * t1118;
    let t44073 = t3311 * t3311;
    let t44075 = t409 / t44073;
    let t44076 = t3314 * t3314;
    let t44077 = F::cast_from(1.0_f64) / t44076;
    let t44080 = F::cast_from(0.24955700379505800916e5_f64) * t44075 * t43970 * t44077;
    let t44082 = F::cast_from(24.0_f64) * t11185 * t11427;
    (t44072, t44080, t44082)
}
