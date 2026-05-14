//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 541/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk541<F: Float>(t2215: F, t735: F, t177: F, t727: F, t737: F, t186: F, t209: F, t660: F) -> (F, F, F, F) {
    let t2217 = 0.17315859105681463759e2 * t735 * t2215;
    let t2218 = t727 * t177;
    let t2219 = t2218 * t737;
    let t2222 = t660 * t209 * t186;
    (t2217, t2218, t2219, t2222)
}
