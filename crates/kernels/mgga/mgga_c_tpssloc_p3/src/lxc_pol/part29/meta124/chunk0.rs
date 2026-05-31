//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 744/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk744<F: Float>(t2765: F, t2766: F, t2773: F, t2778: F, t2782: F, t291: F, t888: F, t892: F, t914: F, t287: F, t891: F, t275: F) -> (F, F, F, F, F, F) {
    let t2784 = t2765 + F::cast_from(0.11872222222222222222e-1_f64) * t2766 - F::cast_from(0.11872222222222222222e-1_f64) * t2773 + F::cast_from(0.35616666666666666666e-1_f64) * t2778 - F::cast_from(0.17808333333333333333e-1_f64) * t2782;
    let t2786 = F::cast_from(0.621814e-1_f64) * t2784 * t291;
    let t2787 = t888 * t892;
    let t2789 = F::cast_from(2.0_f64) * t2787 * t914;
    let t2790 = t891 * t287;
    let t2791 = F::cast_from(1.0_f64) / t2790;
    let t2792 = t275 * t2791;
    (t2784, t2786, t2787, t2789, t2791, t2792)
}
