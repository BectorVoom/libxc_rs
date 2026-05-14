//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 228/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk228<F: Float>(t158: F, t725: F, t162: F, t691: F, t187: F, t192: F, t72: F, t186: F, t650: F, t660: F) -> (F, F, F, F, F) {
    let t726 = t158 * t725;
    let t727 = t691 * t162;
    let t729 = 0.19751673498613801407e-1 * t727 * t187;
    let t730 = t192 * t72;
    let t732 = t660 * t650 * t186;
    (t726, t727, t729, t730, t732)
}
