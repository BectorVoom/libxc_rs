//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 931/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk931<F: Float>(t1338: F, t6434: F, t562: F, t6414: F, t172: F, t6320: F, t763: F, t1824: F, t1834: F, t6387: F, t118: F, t6330: F, t794: F) -> (F, F, F, F, F, F) {
    let t19657 = t1338 * t6434;
    let t19660 = t562 * t6414;
    let t19681 = t6320 * t172;
    let t19682 = t19681 * t763;
    let t19739 = t1834 * t1824;
    let t19743 = t562 * t6387;
    let t19767 = t118 * t794 * t6330;
    (t19657, t19660, t19682, t19739, t19743, t19767)
}
