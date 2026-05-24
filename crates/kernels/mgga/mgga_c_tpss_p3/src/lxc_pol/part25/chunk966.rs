//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 966/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk966<F: Float>(t4566: F, t577: F, t4570: F, t619: F, t1317: F, t3486: F, t4626: F, t1289: F, t70: F, t72: F, t1679: F, t3431: F) -> (F, F, F, F, F, F) {
    let t13298 = t4566 * t577;
    let t13309 = t4570 * t619;
    let t13312 = t1317 * t3486;
    let t13317 = t4626 * t619;
    let t13321 = t1289 * t70 * t72;
    let t13322 = t1679 * t3431;
    (t13298, t13309, t13312, t13317, t13321, t13322)
}
