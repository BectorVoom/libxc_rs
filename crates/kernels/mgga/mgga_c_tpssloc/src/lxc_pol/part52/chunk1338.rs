//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1338/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1338<F: Float>(t1814: F, t31175: F, t8467: F, t26288: F, t5308: F, t6950: F, t114012: F, t114026: F, t114028: F, t114031: F, t114034: F, t114039: F, t114046: F, t120388: F, t120393: F, t120395: F, t120397: F, t120399: F, t120401: F, t120405: F, t120408: F, t120410: F, t120413: F) -> F {
    let t120416 = t1814 * t31175 * t8467;
    let t120419 = t26288 * t6950 * t5308;
    let t120424 = F::cast_from(0.16149102437656156342e-2_f64) * t120388 + F::new(7.0) / F::new(2304.0) * t114012 + F::cast_from(0.80745512188280781708e-3_f64) * t120393 + t120395 / F::new(384.0) - t120397 / F::new(1536.0) + t120399 / F::new(384.0) + t120401 / F::new(768.0) + t114026 - F::cast_from(0.48447307312968469025e-2_f64) * t120405 - F::cast_from(0.80745512188280781708e-3_f64) * t120408 + F::cast_from(0.56521858531796547196e-2_f64) * t120410 + t120413 / F::new(1536.0) - F::new(7.0) / F::new(2304.0) * t120416 + F::cast_from(0.33913115119077928318e-1_f64) * t120419 + t114028 + F::cast_from(0.80745512188280781708e-3_f64) * t114031 - F::new(7.0) / F::new(2304.0) * t114034 + t114039 + F::cast_from(0.13457585364713463618e-3_f64) * t114046;
    t120424
}
