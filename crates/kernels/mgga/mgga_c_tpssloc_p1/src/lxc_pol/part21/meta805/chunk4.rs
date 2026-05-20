//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2797/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2797<F: Float>(t16957: F, t41011: F, t16662: F, t213: F, t221: F, t41142: F, t41144: F, t41149: F, t41155: F, t41156: F, t41185: F, t41187: F, t41190: F, t41192: F, t41194: F, t41197: F, t4127: F, t46764: F, t46768: F, t776: F) -> (F, F) {
    let t59100 = t41011 * t16957;
    let t59134 = F::cast_from(0.16666666666666666666e-2_f64) * t41142 - F::cast_from(0.39999999999999999998e-1_f64) * t41144 - F::cast_from(0.49999999999999999998e-2_f64) * t41149 + t41155 + F::cast_from(0.11234567901234567901e0_f64) * t41156 - t41185 - F::cast_from(0.12962962962962962963e-1_f64) * t41187 + F::cast_from(0.6574074074074074074e-1_f64) * t41190 - F::cast_from(0.52777777777777777776e-2_f64) * t41192 + F::cast_from(0.38888888888888888889e-1_f64) * t41194 + F::cast_from(0.15833333333333333333e-1_f64) * t41197 - F::cast_from(0.39999999999999999998e-1_f64) * t46764 + F::cast_from(0.66666666666666666664e-2_f64) * t46768 + F::cast_from(0.99999999999999999996e-2_f64) * t4127 * t221 * t213 * t16662 * t776;
    (t59100, t59134)
}
