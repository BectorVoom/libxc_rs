//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2906/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2906<F: Float>(t41656: F, t41658: F, t41675: F, t41684: F, t41863: F, t41870: F, t41872: F, t47738: F, t48103: F, t48116: F, t59655: F, t60091: F, t60150: F, t60153: F, t60156: F) -> F {
    let t60601 = F::new(0.20659e1) * t47738 + F::cast_from(0.61745185185185185187e0_f64) * t48103 - F::cast_from(0.22954444444444444444e0_f64) * t41656 - F::cast_from(0.15302962962962962963e0_f64) * t41658 + F::cast_from(0.45908888888888888888e0_f64) * t41675 + F::cast_from(0.10712074074074074074e1_f64) * t41684 + F::cast_from(0.61745185185185185184e0_f64) * t41863 - F::cast_from(0.11577222222222222222e0_f64) * t41870 - F::cast_from(0.3859074074074074074e-1_f64) * t41872 + F::cast_from(0.61745185185185185187e-1_f64) * t48116 - F::new(0.250068e1) * t60091 - F::new(0.123954e2) * t59655 + F::new(0.6311625e0) * t60150 + F::cast_from(0.83356000000000000001e0_f64) * t60153 - F::cast_from(0.18523555555555555556e0_f64) * t60156;
    t60601
}
