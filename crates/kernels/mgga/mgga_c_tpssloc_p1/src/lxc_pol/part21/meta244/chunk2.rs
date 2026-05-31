//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1435/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1435<F: Float>(t3339: F, t3346: F, t4721: F, t4770: F, t5973: F, t5977: F, t5981: F, t5993: F, t6000: F, t6006: F, t6008: F, t6012: F, t6015: F, t6018: F) -> F {
    let t6052 = -F::cast_from(0.17648625e1_f64) * t5993 + F::cast_from(0.3529725e1_f64) * t6000 + t3339 - F::cast_from(0.34431666666666666666e0_f64) * t4721 - F::cast_from(0.34431666666666666667e0_f64) * t5973 + F::cast_from(0.103295e1_f64) * t5977 + F::cast_from(0.516475e0_f64) * t5981 + F::cast_from(0.31558125e0_f64) * t6006 + F::cast_from(0.6311625e0_f64) * t6008 + t3346 - F::cast_from(0.13892666666666666667e0_f64) * t4770 - F::cast_from(0.34731666666666666667e-1_f64) * t6012 + F::cast_from(0.20839e0_f64) * t6015 + F::cast_from(0.104195e0_f64) * t6018;
    t6052
}
