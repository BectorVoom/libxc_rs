//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2743/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2743<F: Float>(t57850: F, t57873: F, t157: F, t182: F, t145: F, t185: F, t46125: F, t46128: F, t46130: F, t16576: F, t751: F, t46132: F) -> (F, F, F, F, F, F, F) {
    let t57874 = t57850 + t57873;
    let t57877 = F::cast_from(0.19751673498613801407e-1_f64) * t57874 * t157 * t182;
    let t57879 = t145 * t57874 * t185;
    let t57880 = F::cast_from(0.70178683471615754484e1_f64) * t46125;
    let t57885 = F::cast_from(0.36622894612013090108e-3_f64) * t46128;
    let t57886 = F::cast_from(0.10389515463408878255e3_f64) * t46130;
    let t57887 = t16576 * t751;
    let t57888 = F::new(2.0) * t57887;
    let t57889 = F::cast_from(0.65061487801810439052e-1_f64) * t46132;
    (t57877, t57879, t57880, t57885, t57886, t57888, t57889)
}
