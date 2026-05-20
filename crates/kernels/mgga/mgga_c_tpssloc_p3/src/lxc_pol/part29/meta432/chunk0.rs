//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1727/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1727<F: Float>(t22666: F, t6907: F, t1985: F, t225: F, t6956: F, t562: F, t794: F) -> (F, F, F, F) {
    let t22667 = t22666 * t6907;
    let t22668 = t1985 * t22667;
    let t22670 = t6956 * t225;
    let t22674 = t794 * t562;
    (t22667, t22668, t22670, t22674)
}
