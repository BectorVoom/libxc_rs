//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 849/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk849<F: Float>(t64: F, t7091: F, t159: F, t216: F, t570: F, t66: F, t235: F, t238: F, t242: F, t232: F, t2215: F, t2218: F, t2345: F, t2206: F, t651: F, t2348: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8185 = t64 * t7091;
    let t8186 = t8185 * t159;
    let t8188 = 455.0 / 1296.0 * t8186 * t216;
    let t8199 = 1.0 / t66 / t570;
    let t8200 = t8199 * t235;
    let t8202 = t8200 * t238 * t242;
    let t8204 = 595.0 / 10368.0 * t232 * t8202;
    let t8212 = t2218 * t2215;
    let t8218 = t2218 * t2345;
    let t8220 = t651 * t2206;
    let t8222 = 0.16265371950452609763e-1 * t2348 * t8220;
    (t8186, t8188, t8200, t8202, t8204, t8212, t8218, t8220, t8222)
}
