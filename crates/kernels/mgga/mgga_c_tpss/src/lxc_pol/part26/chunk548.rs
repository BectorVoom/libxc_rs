//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 548/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk548<F: Float>(t650: F, t713: F, t182: F, t712: F, t177: F, t2211: F, t720: F, t2204: F, t2210: F, t2214: F, t123: F, t173: F, t186: F, t2192: F, t2250: F, t2256: F, t2258: F, t2268: F, t2273: F, t2276: F, t2281: F, t2285: F, t2292: F, t2302: F, t2310: F, t262: F, t699: F, t706: F, t714: F, t721: F) -> (F, F, F, F, F, F, F, F) {
    let t2314 = t650 * t713;
    let t2318 = t712 * t182;
    let t2319 = 1.0 / t2318;
    let t2320 = t177 * t2319;
    let t2321 = t2211 * t720;
    let t2324 = t2204 * t720;
    let t2327 = t177 * t2210;
    let t2328 = t2211 * t2214;
    let t2331 = -0.70983522622222222221e-3 * t123 * t2192 * t173 - 0.34246666666666666666e-1 * t262 * t2250 * t706 - 2.0 * t2256 * t2258 + 1.0 * t699 * t2268 + 0.32163958997385070134e2 * t2273 * t2276 + t2281 + t2285 + t2292 - t2302 - t2310 - 0.24415263074675393405e-3 * t123 * t2192 * t186 - 0.10843581300301739842e-1 * t262 * t2314 * t721 - 0.11696447245269292414e1 * t2320 * t2321 + 0.5848223622634646207e0 * t714 * t2324 + 0.17315859105681463759e2 * t2327 * t2328;
    (t2314, t2319, t2320, t2321, t2324, t2327, t2328, t2331)
}
