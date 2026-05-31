//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1610/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1610<F: Float>(t5794: F, t950: F, t5791: F, t10556: F, t10832: F, t13563: F, t13598: F, t14409: F, t14410: F, t17149: F, t17154: F, t17159: F, t17163: F, t17165: F, t17169: F, t17173: F, t17175: F, t17180: F, t17185: F, t17189: F) -> (F, F, F) {
    let t17451 = t5794 * t950;
    let t17454 = t5791 * t950;
    let t17471 = -t10832 - F::cast_from(0.76103703703703703703e-2_f64) * t10556 - F::cast_from(0.1522074074074074074e-1_f64) * t13598 + F::cast_from(0.761037037037037037e-2_f64) * t13563 - t14409 + t14410 + F::cast_from(0.3805185185185185185e-2_f64) * t17149 - F::cast_from(0.19025925925925925925e-1_f64) * t17154 + F::cast_from(0.68493333333333333331e-1_f64) * t17159 - F::cast_from(0.2283111111111111111e-1_f64) * t17163 - F::cast_from(0.11415555555555555555e-1_f64) * t17165 - F::cast_from(0.10274e0_f64) * t17169 + F::cast_from(0.68493333333333333332e-1_f64) * t17173 + F::cast_from(0.57077777777777777777e-2_f64) * t17175 - F::cast_from(0.11415555555555555555e-1_f64) * t17180 + F::cast_from(0.34246666666666666666e-1_f64) * t17185 - F::cast_from(0.17123333333333333333e-1_f64) * t17189;
    (t17451, t17454, t17471)
}
