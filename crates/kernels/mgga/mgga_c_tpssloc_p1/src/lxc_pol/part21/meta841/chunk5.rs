//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3030/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3030<F: Float>(t50826: F, t50828: F, t50834: F, t63291: F, t63296: F, t63300: F, t63304: F, t63306: F, t63308: F, t63313: F, t63317: F, t63323: F) -> F {
    let t63325 = -F::cast_from(0.2283111111111111111e-1_f64) * t63291 + F::cast_from(0.68493333333333333332e-1_f64) * t63296 + F::cast_from(0.34246666666666666666e-1_f64) * t63300 + F::new(0.10274e0) * t63304 + F::cast_from(0.76103703703703703701e-2_f64) * t63306 - F::cast_from(0.12683950617283950617e-1_f64) * t63308 - F::cast_from(0.2283111111111111111e-1_f64) * t63313 - F::cast_from(0.11415555555555555555e-1_f64) * t63317 + F::cast_from(0.3044148148148148148e-1_f64) * t50826 - F::cast_from(0.11415555555555555555e-1_f64) * t50828 - F::cast_from(0.35515061728395061727e-1_f64) * t50834 + F::cast_from(0.761037037037037037e-1_f64) * t63323;
    t63325
}
