//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3134/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3134<F: Float>(t1174: F, t15293: F, t15320: F, t3447: F, t3449: F, t457: F, t460: F, t4733: F, t4908: F, t4919: F, t52122: F, t52124: F, t52170: F, t64851: F, t64858: F, t64870: F, t64874: F, t64878: F, t64881: F, t7319: F, t974: F) -> F {
    let t64883 = -F::cast_from(0.98765432098765432094e-3_f64) * t52122 - F::cast_from(0.16460905349794238682e-2_f64) * t52124 - F::cast_from(0.16666666666666666666e-2_f64) * t1174 * t974 * t457 * t64851 * t460 - F::cast_from(0.55555555555555555554e-3_f64) * t64858 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t4919 * t52170 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4919 * t7319 * t4733 + F::cast_from(0.22222222222222222222e-2_f64) * t3447 * t15320 * t15293 + F::cast_from(0.22222222222222222222e-2_f64) * t3447 * t3449 * t64870 - F::cast_from(0.66666666666666666665e-2_f64) * t3447 * t4908 * t64874 - F::cast_from(0.18106995884773662551e-2_f64) * t64878 + F::cast_from(0.18518518518518518518e-3_f64) * t64881;
    t64883
}
