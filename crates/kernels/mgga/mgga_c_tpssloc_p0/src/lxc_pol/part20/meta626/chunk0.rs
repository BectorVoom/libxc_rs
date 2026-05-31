//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2256/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2256<F: Float>(t41142: F, t41144: F, t41149: F, t41151: F, t41155: F, t41156: F, t41158: F, t41173: F, t41181: F, t41185: F, t41187: F, t12985: F, t9577: F) -> (F, F) {
    let t46759 = F::cast_from(0.49999999999999999998e-2_f64) * t41142 - F::cast_from(0.59999999999999999997e-1_f64) * t41144 - F::cast_from(0.15e-1_f64) * t41149 + F::cast_from(0.38888888888888888888e-2_f64) * t41151 + t41155 + F::cast_from(0.16851851851851851851e0_f64) * t41156 + F::cast_from(0.46666666666666666664e-1_f64) * t41158 + F::cast_from(0.1e-1_f64) * t41173 + F::cast_from(0.83333333333333333332e-3_f64) * t41181 - t41185 - F::cast_from(0.38888888888888888889e-1_f64) * t41187;
    let t46764 = t9577 * t12985;
    (t46759, t46764)
}
