//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2223/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2223<F: Float>(t40626: F, t4199: F, t9919: F, t12887: F, t67: F, t758: F, t9892: F, t13123: F, t9882: F, t9888: F, t118: F, t2375: F, t4095: F) -> (F, F, F, F, F, F, F) {
    let t46120 = F::new(3.0) * t40626;
    let t46125 = t4199 * t9919;
    let t46126 = F::cast_from(0.35089341735807877242e1_f64) * t46125;
    let t46128 = t12887 * t67 * t758;
    let t46129 = F::cast_from(0.54934341918019635162e-3_f64) * t46128;
    let t46130 = t4199 * t9892;
    let t46131 = F::cast_from(0.51947577317044391277e2_f64) * t46130;
    let t46132 = t13123 * t9882;
    let t46133 = F::cast_from(0.32530743900905219526e-1_f64) * t46132;
    let t46134 = t13123 * t9888;
    let t46135 = F::cast_from(0.48159733137676571078e0_f64) * t46134;
    let t46137 = t4095 * t118 * t2375;
    (t46120, t46126, t46129, t46131, t46133, t46135, t46137)
}
