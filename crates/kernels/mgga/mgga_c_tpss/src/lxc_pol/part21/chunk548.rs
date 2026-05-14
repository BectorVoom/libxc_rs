//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 548/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk548<F: Float>(t128: F, t136: F, t2186: F, t2189: F, t667: F, t124: F, t68: F, t138: F) -> (F, F, F, F) {
    let t2195 = 1.0/f64::sqrt(t128);
    let t2196 = t2195 * t136;
    let t2197 = t2196 * t2186;
    let t2199 = t667 * t2189;
    let t2201 = t124 * t68;
    let t2202 = t138 * t2201;
    (t2196, t2197, t2199, t2202)
}
