//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 896/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk896<F: Float>(t2530: F, t841: F, t2529: F, t281: F, t269: F, t159: F, t2761: F, t2193: F, t838: F) -> (F, F, F, F) {
    let t8595 = t841 * t2530;
    let t8599 = F::new(1.0) / t2529 / t281;
    let t8600 = t269 * t8599;
    let t8609 = t159 * t2761;
    let t8616 = t2193 * t838;
    (t8595, t8600, t8609, t8616)
}
