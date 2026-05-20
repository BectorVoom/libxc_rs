//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2186/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2186<F: Float>(t1307: F, t22635: F, t567: F, t6330: F, t90591: F, t28199: F, t6897: F, t794: F, t1985: F, t20009: F, t214: F, t225: F) -> (F, F, F) {
    let t97588 = t90591 * t22635 * t567 * t6330 * t1307;
    let t97599 = t6897 * t794 * t28199;
    let t97604 = t1985 * t214 * t20009 * t225 * t567;
    (t97588, t97599, t97604)
}
