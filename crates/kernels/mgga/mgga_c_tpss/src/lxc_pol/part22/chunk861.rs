//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 861/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk861<F: Float>(t1333: F, t5527: F, t1364: F, t30: F, t1369: F, t5547: F, t1381: F, t5552: F, t1385: F, t5559: F, t1388: F, t1705: F) -> (F, F, F, F, F, F) {
    let t6109 = t5527 * t1333;
    let t6120 = t30 * t1364;
    let t6124 = t5547 * t1369;
    let t6126 = t5552 * t1381;
    let t6128 = t5559 * t1385;
    let t6134 = t1705 * t1388;
    (t6109, t6120, t6124, t6126, t6128, t6134)
}
