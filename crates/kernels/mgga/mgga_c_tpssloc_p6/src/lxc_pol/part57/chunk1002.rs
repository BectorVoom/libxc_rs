//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1002/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1002<F: Float>(t1914: F, t5527: F, t5660: F, t5664: F, t101840: F, t115009: F, t121782: F, t126180: F, t126198: F, t126530: F, t1877: F, t22960: F, t24191: F, t2522: F, t25373: F, t26563: F, t26744: F, t26756: F, t28241: F, t28249: F, t28256: F, t32899: F, t33476: F, t33477: F, t33483: F, t33484: F, t4314: F, t7114: F, t7545: F, t8566: F, t86716: F, t86721: F, t92319: F, t98064: F) -> (F, F, F, F) {
    let t128097 = t1914 * t5527;
    let t128101 = t1914 * t5660;
    let t128110 = t1914 * t5664;
    let t128134 = F::cast_from(3.0_f64) * t4314 * t8566 * t28241 - F::cast_from(3.0_f64) * t26563 * t22960 * t128097 + t26756 * t25373 * t128101 + F::cast_from(2.0_f64) * t26756 * t126198 - t1877 * t26744 * t32899 - F::cast_from(3.0_f64) * t115009 * t28249 - F::cast_from(3.0_f64) * t26756 * t86716 * t128110 - F::cast_from(3.0_f64) * t92319 * t33477 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t8566 * t28256 - t1877 * t121782 * t7545 + F::cast_from(2.0_f64) * t101840 * t33484 - t1877 * t7114 * t126180 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) * t24191 * t86721 * t33476 + F::cast_from(2.0_f64) * t26756 * t98064 * t33483 - t1877 * t7114 * t126530;
    (t128097, t128101, t128110, t128134)
}
