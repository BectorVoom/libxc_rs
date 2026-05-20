//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2098/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2098<F: Float>(t46206: F, t4199: F, t9494: F, t12945: F, t2427: F, t12858: F, t2528: F, t2371: F, t13123: F, t9885: F, t1409: F, t2516: F, t4194: F, t607: F) -> (F, F, F, F, F, F, F) {
    let t46207 = F::new(12.0) * t46206;
    let t46208 = t4199 * t9494;
    let t46217 = t2427 * t12945;
    let t46218 = F::new(12.0) * t46217;
    let t46234 = t12858 * t2528;
    let t46235 = F::cast_from(0.51947577317044391276e2_f64) * t46234;
    let t46236 = t12858 * t2371;
    let t46237 = F::cast_from(0.35089341735807877242e1_f64) * t46236;
    let t46278 = t13123 * t9885;
    let t46291 = t4194 * t2516 * t1409 * t607;
    (t46207, t46208, t46218, t46235, t46237, t46278, t46291)
}
