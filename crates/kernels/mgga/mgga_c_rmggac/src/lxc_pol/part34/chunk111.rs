//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 111/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk111<F: Float>(t215: F, t28: F, t465: F, t140: F, t217: F, t219: F, t205: F, t449: F, t23: F, t453: F, t446: F, t206: F, t207: F) -> (F, F, F, F, F, F, F) {
    let t466 = t215 * t28;
    let t467 = t465 * t466;
    let t469 = t217 * t140 * t219;
    let t470 = t449 * t205;
    let t472 = t23 * t453;
    let t473 = t472 * t446;
    let t476 = 3.0 * t206 * t473 - t207 * t470;
    (t466, t467, t469, t470, t472, t473, t476)
}
