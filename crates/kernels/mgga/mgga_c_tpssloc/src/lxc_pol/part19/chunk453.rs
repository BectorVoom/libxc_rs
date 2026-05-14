//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 453/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk453<F: Float>(t2388: F, t2391: F, t2394: F, t2398: F, t2400: F, t2403: F) -> (F,) {
    let t2405 = -0.42198333333333333333e0 * t2388 + 0.84396666666666666666e0 * t2391 + 0.39862222222222222223e0 * t2394 + 0.68258333333333333333e-1 * t2398 + 0.13651666666666666667e0 * t2400 + 0.13692777777777777778e0 * t2403;
    (t2405,)
}
