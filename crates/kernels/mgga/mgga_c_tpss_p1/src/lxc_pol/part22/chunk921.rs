//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 921/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk921<F: Float>(t2668: F, t917: F, t2473: F, t845: F, t2530: F, t841: F, t2529: F, t281: F, t269: F, t2470: F, t664: F) -> (F, F, F, F, F) {
    let t8588 = t917 * t2668;
    let t8590 = t2473 * t845;
    let t8595 = t841 * t2530;
    let t8599 = F::new(1.0) / t2529 / t281;
    let t8600 = t269 * t8599;
    let t8605 = t664 * t2470;
    (t8588, t8590, t8595, t8600, t8605)
}
