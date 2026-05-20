//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2907/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2907<F: Float>(t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F, t59657: F, t60161: F, t60163: F, t60166: F, t60168: F, t60171: F, t60173: F, t60176: F) -> F {
    let t60618 = -F::new(0.3529725e1) * t60161 + F::cast_from(0.13892666666666666667e0_f64) * t60163 - F::new(0.104195e0) * t60166 + F::cast_from(0.23154444444444444445e0_f64) * t60168 + F::new(0.41678e0) * t60171 - F::cast_from(0.11577222222222222222e0_f64) * t60173 - F::cast_from(0.15302962962962962963e0_f64) * t59657 + F::new(0.3529725e1) * t60176 + F::cast_from(0.9261777777777777778e0_f64) * t48155 - F::cast_from(0.15436296296296296297e0_f64) * t48157 - F::cast_from(0.55570666666666666668e0_f64) * t48159 - F::cast_from(0.27785333333333333334e0_f64) * t48161 - F::cast_from(0.27785333333333333334e0_f64) * t48163 + F::cast_from(0.9261777777777777778e-1_f64) * t48165 + F::cast_from(0.4630888888888888889e-1_f64) * t48167;
    t60618
}
