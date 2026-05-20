//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1428/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1428<F: Float>(t11352: F, t3351: F, t11344: F, t11350: F, t1136: F, t1138: F, t11415: F, t11420: F, t11441: F, t1148: F, t1156: F, t3332: F, t3333: F, t3334: F, t3357: F, t3359: F, t3360: F, t43911: F, t43997: F, t44000: F, t44002: F, t44006: F, t44072: F, t44080: F, t44082: F, t44085: F, t44089: F, t44092: F) -> F {
    let t44131 = t3351 * t11352;
    let t44138 = t43997 + t44000 - t44002 - t44006 - t44072 - t44080 - t44082 + t44085 + t44089 - t44092 + F::new(36.0) * t3357 * t3334 * t3351 - F::new(8.0) * t3332 * t1138 * t11344 - F::cast_from(0.11579025239058625248e4_f64) * t11420 * t3360 * t3351 + F::cast_from(0.3859675079686208416e3_f64) * t11415 * t11441 + F::cast_from(0.12865583598954028054e3_f64) * t3357 * t11344 * t3359 * t1136 + F::cast_from(0.12414243100625616072e5_f64) * t11350 * t44131 * t3333 + F::cast_from(0.5848223622634646207e0_f64) * t1148 * t43911 * t1156;
    t44138
}
