//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1725/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1725<F: Float>(t23041: F, t831: F, t2627: F, t59: F, t240: F, t812: F, t2617: F, t6613: F, t1878: F, t244: F, t2230: F, t6589: F) -> (F, F, F, F, F, F, F) {
    let t23042 = t23041 * t831;
    let t23046 = t2627 * t59;
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23053 = t2617 * t6613;
    let t23056 = t1878 * t244;
    let t23061 = t2230 * t6589;
    (t23042, t23046, t23047, t23048, t23053, t23056, t23061)
}
