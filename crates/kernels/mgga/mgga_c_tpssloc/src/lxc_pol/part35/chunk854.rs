//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 854/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk854<F: Float>(t17624: F, t973: F, t248: F, t3101: F, t5878: F, t3039: F, t3051: F, t5685: F, t1041: F, t4630: F, t4641: F, t5873: F, t3130: F, t376: F, t5866: F, t2970: F, t5824: F) -> (F, F, F, F, F, F, F) {
    let t17625 = t973 * t17624;
    let t17655 = t248 * t3101 * t5878;
    let t17656 = t3039 * t17655;
    let t17659 = t248 * t3051 * t5685;
    let t17660 = t1041 * t17659;
    let t17662 = t4641 * t4630;
    let t17667 = t248 * t3101 * t5873;
    let t17668 = t3130 * t17667;
    let t17712 = t376 * t5866;
    let t17763 = t2970 * t5824;
    (t17625, t17656, t17660, t17662, t17668, t17712, t17763)
}
