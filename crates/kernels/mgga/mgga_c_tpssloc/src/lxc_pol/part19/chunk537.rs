//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 537/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk537<F: Float>(t2765: F, t2766: F, t2773: F, t2778: F, t2782: F, t291: F, t888: F, t892: F, t914: F, t287: F, t891: F, t275: F, t912: F) -> (F, F, F, F, F, F, F, F) {
    let t2784 = t2765 + 0.11872222222222222222e-1 * t2766 - 0.11872222222222222222e-1 * t2773 + 0.35616666666666666666e-1 * t2778 - 0.17808333333333333333e-1 * t2782;
    let t2786 = 0.621814e-1 * t2784 * t291;
    let t2787 = t888 * t892;
    let t2789 = 2.0 * t2787 * t914;
    let t2790 = t891 * t287;
    let t2791 = 1.0 / t2790;
    let t2792 = t275 * t2791;
    let t2793 = t912 * t912;
    (t2784, t2786, t2787, t2789, t2790, t2791, t2792, t2793)
}
