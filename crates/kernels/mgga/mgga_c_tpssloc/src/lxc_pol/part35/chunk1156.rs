//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1156/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1156<F: Float>(t11605: F, t225: F, t11545: F, t461: F, t491: F, t1009: F, t460: F, t27495: F, t1193: F, t24811: F, t3545: F, t7372: F, t131: F, t467: F, t50: F, t82510: F) -> (F, F, F, F, F, F, F) {
    let t85674 = t225 * t11605;
    let t85754 = t11545 * t461;
    let t85755 = t85754 * t491;
    let t85821 = t460 * t1009;
    let t85822 = t85821 * t27495;
    let t85853 = t24811 * t1193;
    let t85909 = t85754 * t225;
    let t85917 = t7372 * t3545;
    let t85963 = t50 * t82510 * t131 * t467;
    (t85674, t85755, t85822, t85853, t85909, t85917, t85963)
}
