//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1920/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1920<F: Float>(t17817: F, t4531: F, t17804: F, t4514: F, t10295: F, t13642: F, t17286: F, t17288: F, t17290: F, t21120: F, t21132: F, t21136: F, t21140: F, t21161: F, t21168: F) -> (F, F, F) {
    let t21430 = t4531 * t17817;
    let t21433 = t17804 * t4514;
    let t21444 = t10295 + F::cast_from(5.0_f64) / F::cast_from(9.0_f64) * t13642 - t17286 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t17288 - t17290 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t21132 - t21120 / F::cast_from(3.0_f64) + t21168 / F::cast_from(6.0_f64) + t21140 - t21161 + t21136 / F::cast_from(6.0_f64);
    (t21430, t21433, t21444)
}
