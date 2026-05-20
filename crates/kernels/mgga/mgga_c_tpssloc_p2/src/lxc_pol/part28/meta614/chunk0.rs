//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1929/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1929<F: Float>(t16261: F, t26309: F, t22832: F, t5234: F, t3809: F, t16405: F, t22833: F, t16387: F, t16275: F, t16271: F, t1336: F, t22759: F, t5252: F, t836: F) -> (F, F, F, F, F, F, F) {
    let t91098 = t26309 * t16261;
    let t91100 = t5234 * t22832;
    let t91101 = t91100 * t3809;
    let t91103 = t22833 * t16405;
    let t91105 = t26309 * t16387;
    let t91107 = t22833 * t16275;
    let t91109 = t22833 * t16271;
    let t91113 = t1336 * t22759 * t836 * t5252;
    (t91098, t91101, t91103, t91105, t91107, t91109, t91113)
}
