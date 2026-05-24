//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1016/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1016<F: Float>(t46427: F, t5148: F, t2064: F, t9908: F, t46501: F, t5259: F, t40826: F, t9704: F, t1587: F, t1614: F, t25820: F, t305: F, t321: F, t333: F, t36058: F, t40983: F, t41059: F, t45527: F, t46575: F, t46582: F, t4669: F, t46694: F, t558: F, t570: F, t8975: F) -> F {
    let t46702 = t5148 * t46427;
    let t46707 = t9908 * t2064;
    let t46710 = t5259 * t46501;
    let t46715 = t40826 * t9704;
    let t46734 = F::cast_from(0.2993560425465952141e-1_f64) * t46702 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t46694 * t321 - F::cast_from(0.79828278012425390427e-1_f64) * t46707 - F::cast_from(0.14635184302277988245e0_f64) * t36058 - F::cast_from(0.5987120850931904282e-1_f64) * t46710 - F::cast_from(0.35922725105591425692e0_f64) * t25820 * t46575 * t333 - F::cast_from(0.5987120850931904282e-1_f64) * t46715 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t8975 * t1614 + F::cast_from(0.23948483403727617128e0_f64) * t5259 * t8975 * t1587 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t41059 * t570 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t40983 * t558 + F::cast_from(0.11974241701863808564e0_f64) * t305 * t45527 + F::cast_from(0.23948483403727617128e0_f64) * t5259 * t46582 * t321;
    t46734
}
