//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2262/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2262<F: Float>(t13007: F, t46843: F, t131: F, t205: F, t41160: F, t116: F, t212: F, t2570: F, t2585: F, t4255: F, t12988: F, t13005: F, t221: F, t2379: F, t41209: F, t41212: F, t41217: F, t4127: F, t4128: F, t46828: F, t46830: F, t46836: F, t46838: F, t46839: F, t9458: F, t9516: F) -> (F, F) {
    let t46844 = t46843 * t13007;
    let t46847 = t205 * t41160 * t131;
    let t46853 = t116 * t212;
    let t46855 = t2585 * t2570 * t46853 * t4255;
    let t46856 = F::cast_from(0.14999999999999999999e-1_f64) * t46855;
    let t46858 = F::cast_from(0.49999999999999999998e-2_f64) * t4127 * t221 * t4128 * t9516 - F::cast_from(0.74999999999999999997e-2_f64) * t46828 - F::cast_from(0.69999999999999999996e-1_f64) * t46830 - F::cast_from(0.59999999999999999997e-1_f64) * t13005 * t221 * t12988 * t2379 - F::cast_from(0.34999999999999999998e-1_f64) * t46836 - F::cast_from(0.59999999999999999997e-1_f64) * t13005 * t46838 * t46839 + F::cast_from(0.13999999999999999999e0_f64) * t46844 + F::cast_from(0.99999999999999999995e-1_f64) * t46847 * t221 * t4128 * t9458 - t46856 + t41209 + t41212 + F::cast_from(0.83333333333333333331e-3_f64) * t41217;
    (t46853, t46858)
}
