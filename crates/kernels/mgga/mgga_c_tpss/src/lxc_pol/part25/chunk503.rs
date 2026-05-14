//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 503/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk503<F: Float>(t226: F, t750: F, t128: F, t131: F, t136: F, t124: F, t137: F, t68: F, t209: F, t660: F, t659: F, t125: F) -> (F, F, F, F, F, F, F, F) {
    let t2177 = t226 * t750;
    let t2184 = 1.0 / t131 / t128 * t136;
    let t2185 = t137 * t124;
    let t2186 = t2185 * t68;
    let t2187 = t2184 * t2186;
    let t2189 = t660 * t209;
    let t2190 = t659 * t2189;
    let t2192 = t125 * t209;
    (t2177, t2184, t2185, t2186, t2187, t2189, t2190, t2192)
}
