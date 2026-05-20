//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2260/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2260<F: Float>(t12984: F, t2379: F, t46799: F, t686: F, t133: F, t1484: F, t41214: F, t6600: F, t12998: F, t46766: F, t776: F, t12971: F, t12988: F, t213: F, t221: F, t2553: F, t41203: F, t41205: F, t4127: F, t46788: F, t46790: F, t46794: F, t46796: F) -> F {
    let t46802 = t46799 * t686 * t12984 * t2379;
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46819 = t12998 * t686 * t46766 * t776;
    let t46821 = -F::new(0.75e-2) * t41203 - F::cast_from(0.34999999999999999998e-1_f64) * t41205 + F::cast_from(0.11666666666666666666e-1_f64) * t46788 + F::cast_from(0.56172839506172839502e-1_f64) * t46790 + t46794 + F::cast_from(0.47499999999999999998e-1_f64) * t46796 + F::cast_from(0.29999999999999999998e-1_f64) * t46802 + F::cast_from(0.27777777777777777777e-3_f64) * t46806 + F::cast_from(0.14999999999999999999e-1_f64) * t4127 * t221 * t213 * t12971 * t776 + F::cast_from(0.14999999999999999999e-1_f64) * t4127 * t221 * t12988 * t2553 - F::cast_from(0.14999999999999999999e-1_f64) * t46819;
    t46821
}
