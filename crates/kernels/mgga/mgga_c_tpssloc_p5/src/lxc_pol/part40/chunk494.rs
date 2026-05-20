//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 494/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk494<F: Float>(t265: F, t504: F, t1238: F, t1721: F, t1752: F, t1761: F, t498: F, t1256: F, t1534: F, t1659: F, t1673: F, t1699: F, t1701: F, t1705: F, t193: F, t336: F) -> (F, F) {
    let t505 = t265 < t504;
    let t1763 = -t1238 * t1761 + t1721 * t498 + t1752 * t498;
    let t1768 = piecewise3::<F>(t505, t1256 * t1763 * t193 * t336 - t1659 + t1673 + t1699 + t1701 - t1705, t1534);
    (t1763, t1768)
}
