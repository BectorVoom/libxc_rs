//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 271/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk271<F: Float>(t849: F, t854: F, t235: F, t671: F, t275: F, t277: F, t334: F) -> (F, F, F, F, F) {
    let t855 = t854 * t849;
    let t857 = t671 * t235;
    let t859 = t275 * t857 * t277;
    let t860 = F::new(0.82156666666666666667e-1) * t859;
    let t861 = t235 * t334;
    (t855, t857, t859, t860, t861)
}
