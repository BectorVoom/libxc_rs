//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2052/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2052<F: Float>(t1354: F, t16288: F, t12211: F, t5223: F, t1307: F, t210: F, t5226: F, t1810: F, t3719: F, t3804: F, t820: F) -> (F, F, F, F, F) {
    let t16290 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t16288 * t1354;
    let t16294 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t12211 * t5223;
    let t16296 = t210 * t5226 * t1307;
    let t16300 = t210 * t1810 * t3719;
    let t16305 = t3804 * t820;
    (t16290, t16294, t16296, t16300, t16305)
}
