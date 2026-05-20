//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 531/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk531<F: Float>(t1720: F, t491: F, t1196: F, t1409: F, t974: F, t225: F, t68: F, t484: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F) -> (F, F, F, F, F, F, F) {
    let t1721 = t1720 * t491;
    let t1725 = t1196 * t1409;
    let t1726 = t974 * t1725;
    let t1729 = t1720 * t225;
    let t1730 = t1729 * t68;
    let t1731 = t1730 * t484;
    let t1734 = -t1659 + t1673 + t1699 + t1701 - t1705;
    (t1721, t1725, t1726, t1729, t1730, t1731, t1734)
}
