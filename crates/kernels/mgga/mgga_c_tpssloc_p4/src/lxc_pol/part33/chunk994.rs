//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 994/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk994<F: Float>(t21347: F, t324: F, t10832: F, t13598: F, t17149: F, t17165: F, t17175: F, t21124: F, t21128: F, t21147: F, t21150: F, t21153: F, t21156: F) -> (F, F) {
    let t21348 = t21347 * t324;
    let t21360 = -t10832 - F::cast_from(0.2283111111111111111e-1_f64) * t13598 + F::cast_from(0.11415555555555555555e-1_f64) * t17149 - F::cast_from(0.34246666666666666665e-1_f64) * t17165 + F::cast_from(0.17123333333333333333e-1_f64) * t17175 - F::cast_from(0.19025925925925925925e-1_f64) * t21147 + F::cast_from(0.68493333333333333331e-1_f64) * t21150 - F::cast_from(0.34246666666666666665e-1_f64) * t21124 - F::new(0.10274e0) * t21153 + F::new(0.10274e0) * t21128 - F::cast_from(0.17123333333333333333e-1_f64) * t21156;
    (t21348, t21360)
}
